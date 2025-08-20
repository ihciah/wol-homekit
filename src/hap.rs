use std::{
    net::{IpAddr, Ipv4Addr},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use hap::{
    accessory::{switch, AccessoryCategory, AccessoryInformation},
    characteristic::CharacteristicCallbacks,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    Config, Error as HapError, MacAddress, Pin,
};

use crate::wol::MagicPacket;

#[derive(Debug, Clone)]
struct DelaySwitch(Arc<Mutex<Inner>>);

#[derive(Debug, Clone)]
struct Inner {
    last: Option<Instant>,
    packet: MagicPacket,
}

impl DelaySwitch {
    fn new(mac_address: &[u8; 6], interface: Option<String>) -> Self {
        Self(Arc::new(Mutex::new(Inner {
            last: None,
            packet: MagicPacket::new(mac_address, interface),
        })))
    }

    fn get(&self) -> bool {
        let locked = self.0.lock().unwrap();
        locked
            .last
            .map(|instant| instant.elapsed() < Duration::from_secs(10))
            .unwrap_or(false)
    }

    fn set(&self) {
        let mut inner = self.0.lock().unwrap();
        inner.last = Some(Instant::now());
        for _ in 0..3 {
            if let Err(e) = inner.packet.send() {
                tracing::error!("send wol packet fail: {e}");
            }
        }
    }

    fn clear(&self) {
        self.0.lock().unwrap().last = None;
    }
}

pub struct Accessory {
    server: IpServer,
}

impl Deref for Accessory {
    type Target = IpServer;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl DerefMut for Accessory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.server
    }
}

impl Accessory {
    pub async fn new(
        info: AccessoryInformation,
        mut storage: FileStorage,
        pin: &[u8; 8],
        target_mac: &[u8; 6],
        interface: Option<String>,
    ) -> Result<Self, HapError> {
        let name = info.name.clone();
        let mut switch = switch::SwitchAccessory::new(1, info)?;
        let delay_read = DelaySwitch::new(target_mac, interface);
        let delay_write = delay_read.clone();
        switch
            .switch
            .power_state
            .on_read(Some(move || Ok(Some(delay_read.get()))));
        switch
            .switch
            .power_state
            .on_update(Some(move |_current_val: &bool, new_val: &bool| {
                match *new_val {
                    true => {
                        delay_write.set();
                    }
                    false => {
                        delay_write.clear();
                    }
                }
                Ok(())
            }));
        const ANY: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let config = match storage.load_config().await {
            Ok(mut config) => {
                config.host = ANY;
                storage.save_config(&config).await?;
                config
            }
            Err(_) => {
                let config = Config {
                    host: ANY,
                    pin: Pin::new(*pin)?,
                    name,
                    device_id: MacAddress::from([32, 54, 67, 87, 12, 45]),
                    category: AccessoryCategory::Switch,
                    ..Default::default()
                };
                storage.save_config(&config).await?;
                config
            }
        };

        let server = IpServer::new(config, storage).await?;
        server.add_accessory(switch).await?;
        Ok(Self { server })
    }
}
