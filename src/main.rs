use hap::{accessory::AccessoryInformation, server::Server, storage::FileStorage};
use wol::hap::Accessory;

const TARGET_MAC: &[u8; 6] = &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
const INTERFACE: &str = "eth0";
const PIN: &[u8; 8] = &[1, 4, 7, 7, 4, 1, 4, 7];

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let info = AccessoryInformation {
        name: "WOL".into(),
        manufacturer: "ihciah".into(),
        model: env!("CARGO_PKG_VERSION").into(),
        ..Default::default()
    };
    let storage = FileStorage::current_dir().await.unwrap();
    let accessory = Accessory::new(info, storage, PIN, TARGET_MAC, Some(INTERFACE.into()))
        .await
        .unwrap();
    tracing::info!("WOL started");
    accessory.run_handle().await.unwrap();
}
