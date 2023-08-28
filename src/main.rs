use ruuvi_reader_rs::scanner::read_events;

#[tokio::main]
async fn main() {
    read_events().await.unwrap();
}
