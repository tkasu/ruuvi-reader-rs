use async_trait::async_trait;
use ruuvi_sensor_protocol::SensorValues;

use crate::provider::TelemetryProvider;

/// Mock telemetry provider for testing
pub struct MockProvider {
    pub data: Vec<SensorValues>,
    index: usize,
}

impl MockProvider {
    /// Create a new mock provider with predefined sensor values
    pub fn new(data: Vec<SensorValues>) -> Self {
        MockProvider { data, index: 0 }
    }

    /// Create a mock provider with sample test data
    pub fn with_sample_data() -> Self {
        let samples = vec![
            create_sample_sensor_values(
                [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x11],
                22005,  // 22.005°C
                570925, // 57.0925% humidity
                100621, // 100621 Pa
                2941,   // 2941 mV
                4,      // 4 dBm
                10,
                100,
            ),
            create_sample_sensor_values(
                [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x22],
                -5000,  // -5°C
                450000, // 45% humidity
                101325, // 101325 Pa (sea level)
                3000,   // 3000 mV
                0,      // 0 dBm
                5,
                101,
            ),
            create_sample_sensor_values(
                [0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA],
                35500,  // 35.5°C
                800000, // 80% humidity
                99000,  // 99000 Pa
                2500,   // 2500 mV (low battery)
                8,      // 8 dBm
                250,
                102,
            ),
        ];

        MockProvider::new(samples)
    }

    /// Create an empty mock provider that immediately returns None
    pub fn empty() -> Self {
        MockProvider::new(vec![])
    }
}

#[async_trait]
impl TelemetryProvider for MockProvider {
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Reset index when starting
        self.index = 0;
        Ok(())
    }

    async fn next(&mut self) -> Option<SensorValues> {
        if self.index < self.data.len() {
            let value = self.data[self.index].clone();
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

/// Helper function to create sample sensor values for testing
/// This creates a valid Ruuvi Data Format 5 payload
fn create_sample_sensor_values(
    mac: [u8; 6],
    temp_millicelsius: i32,
    humidity_ppm: u32,
    pressure_pa: u32,
    battery_mv: u16,
    tx_power_dbm: i8,
    movement: u32,
    sequence: u32,
) -> SensorValues {
    // Create a Ruuvi Data Format 5 payload
    // Format: 0x05 (1 byte) + temperature (2 bytes) + humidity (2 bytes) + pressure (2 bytes) +
    //         acceleration_x (2 bytes) + acceleration_y (2 bytes) + acceleration_z (2 bytes) +
    //         power (2 bytes: 11 bits battery, 5 bits tx_power) + movement (1 byte) +
    //         sequence (2 bytes) + mac (6 bytes)

    let mut payload = vec![0x05]; // Format version 5

    // Temperature: signed 16-bit integer, 0.005°C steps
    let temp_raw = (temp_millicelsius as f32 / 5.0) as i16;
    payload.extend_from_slice(&temp_raw.to_be_bytes());

    // Humidity: unsigned 16-bit integer, 0.0025% steps
    let humidity_raw = (humidity_ppm as f32 / 25.0) as u16;
    payload.extend_from_slice(&humidity_raw.to_be_bytes());

    // Pressure: unsigned 16-bit integer, Pa - 50000, 1 Pa steps
    let pressure_raw = (pressure_pa - 50000) as u16;
    payload.extend_from_slice(&pressure_raw.to_be_bytes());

    // Acceleration (dummy values)
    payload.extend_from_slice(&[0x00, 0x00]); // x
    payload.extend_from_slice(&[0x00, 0x00]); // y
    payload.extend_from_slice(&[0x00, 0x00]); // z

    // Power info: 11 bits battery + 5 bits tx_power
    let battery_raw = battery_mv + 1600; // offset
    let tx_power_raw = ((tx_power_dbm + 40) / 2) as u16; // convert to 5-bit value
    let power_info = ((battery_raw << 5) | (tx_power_raw & 0x1F)) as u16;
    payload.extend_from_slice(&power_info.to_be_bytes());

    // Movement counter (1 byte)
    payload.push(movement as u8);

    // Measurement sequence number (2 bytes)
    payload.extend_from_slice(&(sequence as u16).to_be_bytes());

    // MAC address (6 bytes)
    payload.extend_from_slice(&mac);

    // Parse using the manufacturer ID for Ruuvi (0x0499)
    SensorValues::from_manufacturer_specific_data(0x0499, &payload)
        .expect("Failed to create valid sensor values")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruuvi_sensor_protocol::{Humidity, MacAddress, Temperature};

    #[tokio::test]
    async fn test_mock_provider_with_sample_data() {
        let mut provider = MockProvider::with_sample_data();
        provider.start().await.unwrap();

        // Should get 3 samples
        let first = provider.next().await;
        assert!(first.is_some());
        let first_value = first.unwrap();
        assert_eq!(first_value.temperature_as_millicelsius(), Some(22005));

        let second = provider.next().await;
        assert!(second.is_some());
        let second_value = second.unwrap();
        assert_eq!(second_value.temperature_as_millicelsius(), Some(-5000));

        let third = provider.next().await;
        assert!(third.is_some());
        let third_value = third.unwrap();
        assert_eq!(third_value.temperature_as_millicelsius(), Some(35500));

        // Fourth call should return None
        let fourth = provider.next().await;
        assert!(fourth.is_none());
    }

    #[tokio::test]
    async fn test_mock_provider_empty() {
        let mut provider = MockProvider::empty();
        provider.start().await.unwrap();

        let result = provider.next().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_mock_provider_restart() {
        let mut provider = MockProvider::with_sample_data();
        provider.start().await.unwrap();

        // Consume all items
        while provider.next().await.is_some() {}

        // Restart should reset
        provider.start().await.unwrap();
        let first_again = provider.next().await;
        assert!(first_again.is_some());
    }

    #[tokio::test]
    async fn test_mock_provider_custom_data() {
        let custom_data = vec![create_sample_sensor_values(
            [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            25000,
            600000,
            101000,
            3000,
            4,
            1,
            1,
        )];

        let mut provider = MockProvider::new(custom_data);
        provider.start().await.unwrap();

        let value = provider.next().await.unwrap();
        assert_eq!(value.mac_address(), Some([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]));
        assert_eq!(value.temperature_as_millicelsius(), Some(25000));
        assert_eq!(value.humidity_as_ppm(), Some(600000));
    }
}
