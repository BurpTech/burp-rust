use std::error::Error;
use std::str::{from_utf8, Utf8Error};
use std::sync::{Arc, Mutex};

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use heapless::String;
use log::*;
use thiserror::Error;

use crate::config::Config;
use crate::name::get_name;
use crate::traits::mdns::Mdns;
use crate::traits::storage::Storage;
use crate::traits::wifi::Wifi;

pub struct Network<'a, S, W, M> {
    config: Arc<Mutex<Config<'a, S>>>,
    wifi: W,
    mdns: M,
}

#[derive(Error, Debug)]
pub enum NetworkError<W: Error, M: Error> {
    Utf8Error(Utf8Error),
    WifiError(W),
    MdnsError(M),
}

impl<S: Storage, W: Wifi, M: Mdns> Network<'_, S, W, M> {
    pub fn new(
        config: Arc<Mutex<Config<S>>>,
        wifi: W,
        mdns: M,
    ) -> Network<S, W, M> {
        Network {
            config,
            wifi,
            mdns,
        }
    }

    pub async fn start(&mut self) -> Result<(), NetworkError<W::Error, M::Error>> {
        let ssid = self.get_ssid().map_err(NetworkError::Utf8Error)?;
        let password = self.get_password().map_err(NetworkError::Utf8Error)?;
        self.start_wifi(ssid, password).await.map_err(NetworkError::WifiError)?;
        self.start_mdns().map_err(NetworkError::MdnsError)?;
        Ok(())
    }

    fn get_ssid(&self) -> Result<String<32>, Utf8Error> {
        Ok(String::from(
            from_utf8(self.config.lock().unwrap().ssid.get())?
        ))
    }

    fn get_password(&self) -> Result<String<64>, Utf8Error> {
        Ok(String::from(
            from_utf8(self.config.lock().unwrap().psk.get())?
        ))
    }

    async fn start_wifi(&mut self, ssid: String<32>, password: String<64>) -> Result<(), W::Error> {
        self.wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;
        self.wifi.start().await?;
        info!("Wifi scanning for ssid: {}", ssid);
        let ap_infos = self.wifi.scan().await?;
        let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);
        let channel = if let Some(ours) = ours {
            info!("Found configured access point {} on channel {}", ssid, ours.channel);
            Some(ours.channel)
        } else {
            info!("Configured access point {} not found during scanning, will go with unknown channel", ssid);
            None
        };

        self.wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid,
            password,
            channel,
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        }))?;

        info!("Connecting wifi...");
        self.wifi.connect().await?;
        info!("Waiting for DHCP lease...");
        self.wifi.wait_netif_up().await?;
        let ip_info = self.wifi.get_ip_info()?;
        info!("Wifi DHCP info: {:?}", ip_info);
        Ok(())
    }

    fn start_mdns(&mut self) -> Result<(), M::Error> {
        let name = get_name();
        info!("Setting MDNS hostname");
        self.mdns.set_hostname(name)?;
        info!("Setting MDNS instance name");
        self.mdns.set_instance_name(name)?;
        info!("Adding _burptech service");
        self.mdns.add_service(Some(name), "_burptech", "_tcp", 1234, &[])?;
        Ok(())
    }
}
