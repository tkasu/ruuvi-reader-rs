use async_trait::async_trait;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::{Stream, StreamExt};
use ruuvi_sensor_protocol::SensorValues;
use std::pin::Pin;

use crate::provider::TelemetryProvider;

pub struct BleProvider {
    events: Option<Pin<Box<dyn Stream<Item = CentralEvent> + Send>>>,
    adapter: Option<Adapter>,
}

impl BleProvider {
    pub fn new() -> Self {
        BleProvider {
            events: None,
            adapter: None,
        }
    }

    async fn get_central(manager: &Manager) -> Result<Adapter, btleplug::Error> {
        let adapters = manager.adapters().await?;
        adapters
            .into_iter()
            .next()
            .ok_or_else(|| btleplug::Error::Other("No Bluetooth adapters found".into()))
    }
}

#[async_trait]
impl TelemetryProvider for BleProvider {
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let manager = Manager::new().await?;
        let central = Self::get_central(&manager).await?;

        let events = central.events().await?;
        central.start_scan(ScanFilter::default()).await?;

        self.events = Some(Box::pin(events));
        self.adapter = Some(central);

        Ok(())
    }

    async fn next(&mut self) -> Option<SensorValues> {
        let events = self.events.as_mut()?;

        loop {
            match events.next().await {
                Some(CentralEvent::ManufacturerDataAdvertisement {
                    id: _id,
                    manufacturer_data,
                    ..
                }) => {
                    if let Some((man_id, data)) = manufacturer_data.iter().next() {
                        if let Ok(sensor_values) =
                            SensorValues::from_manufacturer_specific_data(*man_id, data)
                        {
                            return Some(sensor_values);
                        }
                    }
                    // If parsing failed, continue to next event
                }
                Some(_) => {
                    // Other events, continue
                }
                None => {
                    // Stream ended
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ble_provider_creation() {
        let provider = BleProvider::new();
        assert!(provider.events.is_none());
        assert!(provider.adapter.is_none());
    }
}
