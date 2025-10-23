use ruuvi_reader_rs::ble_provider::BleProvider;
use ruuvi_reader_rs::scanner::read_events;

#[tokio::main]
async fn main() {
    let mut provider = BleProvider::new();
    read_events(&mut provider).await.unwrap();
}
