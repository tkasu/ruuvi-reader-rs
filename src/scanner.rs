use btleplug;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use ruuvi_sensor_protocol::{
    BatteryPotential, Humidity, MacAddress, MeasurementSequenceNumber, MovementCounter, Pressure,
    SensorValues, Temperature, TransmitterPower,
};
use serde::Serialize;
use serde_json;
use serde_json::json;

// Wrap ruuvi_sensor_protocol's SensorValues to provide a Serialize implementation
#[derive(Debug, Serialize)]
struct SensorValuesDef {
    mac_address: Option<[u8; 6]>,
    humidity: Option<u32>,
    temperature_millicelsius: Option<i32>,
    pressure: Option<u32>,
    battery_potential: Option<u16>,
    tx_power: Option<i8>,
    movement_counter: Option<u32>,
    measurement_sequence_number: Option<u32>,
}

impl From<SensorValues> for SensorValuesDef {
    fn from(protocol_values: SensorValues) -> SensorValuesDef {
        SensorValuesDef {
            humidity: protocol_values.humidity_as_ppm(),
            temperature_millicelsius: protocol_values.temperature_as_millicelsius(),
            pressure: protocol_values.pressure_as_pascals(),
            battery_potential: protocol_values.battery_potential_as_millivolts(),
            tx_power: protocol_values.tx_power_as_dbm(),
            movement_counter: protocol_values.movement_counter(),
            measurement_sequence_number: protocol_values.measurement_sequence_number(),
            mac_address: protocol_values.mac_address(),
        }
    }
}

pub async fn read_events() -> Result<(), btleplug::Error> {
    let manager = Manager::new().await?;
    let central = get_central(&manager).await;

    let mut events = central.events().await?;
    central.start_scan(ScanFilter::default()).await?;
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::ManufacturerDataAdvertisement {
                id: _id,
                manufacturer_data,
                ..
            } => {
                let (man_id, data) = manufacturer_data.iter().next().unwrap();
                let parsed = SensorValues::from_manufacturer_specific_data(man_id.clone(), data);
                match parsed {
                    Ok(sensor_values) => {
                        let json = json!(SensorValuesDef::from(sensor_values));
                        println!("{}", json.to_string());
                    }
                    Err(_) => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}
