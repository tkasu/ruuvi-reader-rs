use async_trait::async_trait;
use ruuvi_sensor_protocol::SensorValues;

/// Trait for providing telemetry data from Ruuvi sensors
#[async_trait]
pub trait TelemetryProvider {
    /// Start the provider and begin yielding sensor values
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Get the next sensor value, or None if the stream has ended
    async fn next(&mut self) -> Option<SensorValues>;
}
