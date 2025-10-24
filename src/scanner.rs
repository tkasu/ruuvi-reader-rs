use ruuvi_sensor_protocol::{
    BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber, MovementCounter, Pressure,
    SensorValues, Temperature, TransmitterPower,
};
use serde::Serialize;
use serde_json::json;

use crate::provider::TelemetryProvider;

// Wrap ruuvi_sensor_protocol's SensorValues to provide a Serialize implementation
#[derive(Debug, Serialize)]
pub struct SensorValuesDef {
    pub mac_address: Option<[u8; 6]>,
    pub humidity: Option<u32>,
    pub temperature_millicelsius: Option<i32>,
    pub pressure: Option<u32>,
    pub battery_potential: Option<u16>,
    pub tx_power: Option<i8>,
    pub movement_counter: Option<u32>,
    pub measurement_sequence_number: Option<u32>,
    pub measurement_ts_ms: u128,
}

impl From<SensorValues> for SensorValuesDef {
    fn from(protocol_values: SensorValues) -> SensorValuesDef {
        let now_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        SensorValuesDef {
            humidity: protocol_values.humidity_as_ppm(),
            temperature_millicelsius: protocol_values.temperature_as_millicelsius(),
            pressure: protocol_values.pressure_as_pascals(),
            battery_potential: protocol_values.battery_potential_as_millivolts(),
            tx_power: protocol_values.tx_power_as_dbm(),
            movement_counter: protocol_values.movement_counter(),
            measurement_sequence_number: protocol_values.measurement_sequence_number(),
            mac_address: protocol_values.mac_address(),
            measurement_ts_ms: now_epoch,
        }
    }
}

/// Read events from a telemetry provider and output them as JSON lines
pub async fn read_events<P: TelemetryProvider>(
    provider: &mut P,
) -> Result<(), Box<dyn std::error::Error>> {
    provider.start().await?;

    while let Some(sensor_values) = provider.next().await {
        let json = json!(SensorValuesDef::from(sensor_values));
        println!("{}", json);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_provider::MockProvider;

    #[tokio::test]
    async fn test_read_events_with_mock_provider() {
        let mut provider = MockProvider::with_sample_data();
        let result = read_events(&mut provider).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_events_with_empty_provider() {
        let mut provider = MockProvider::empty();
        let result = read_events(&mut provider).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_sensor_values_def_conversion() {
        use crate::mock_provider::MockProvider;

        let provider = MockProvider::with_sample_data();
        let sample = provider.data[0].clone();

        let def = SensorValuesDef::from(sample.clone());

        assert_eq!(
            def.temperature_millicelsius,
            sample.temperature_as_millicelsius()
        );
        assert_eq!(def.humidity, sample.humidity_as_ppm());
        assert_eq!(def.pressure, sample.pressure_as_pascals());
        assert_eq!(
            def.battery_potential,
            sample.battery_potential_as_millivolts()
        );
        assert_eq!(def.tx_power, sample.tx_power_as_dbm());
        assert_eq!(def.movement_counter, sample.movement_counter());
        assert_eq!(
            def.measurement_sequence_number,
            sample.measurement_sequence_number()
        );
        assert_eq!(def.mac_address, sample.mac_address());
    }
}
