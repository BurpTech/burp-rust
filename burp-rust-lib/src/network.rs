use std::error::Error;
use std::str::from_utf8;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use crate::config::Config;
use crate::traits::mdns::Mdns;
use crate::traits::wifi::Wifi;
use log::*;

pub struct Network<'a, E: Error> {
    config: &'a Config<'a>,
    wifi: &'a mut dyn Wifi<E>,
    mdns: &'a mut dyn Mdns<E>,
}

impl<E: Error> Network<'_, E> {
    pub fn new<'a>(
        config: &'a Config,
        wifi: &'a mut dyn Wifi<E>,
        mdns: &'a mut dyn Mdns<E>,
    ) -> Network<'a, E> {
        Network {
            config,
            wifi,
            mdns,
        }
    }

    pub async fn start(&mut self, ) -> Result<(), E> {
        let ssid = from_utf8(self.config.ssid.get()).unwrap();
        let pass = from_utf8(self.config.psk.get()).unwrap();

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
        let auth_method = AuthMethod::WPA2Personal;
        self.wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: ssid.into(),
            password: pass.into(),
            channel,
            auth_method,
            ..Default::default()
        }))?;
        info!("Connecting wifi...");
        self.wifi.connect().await?;
        info!("Waiting for DHCP lease...");
        self.wifi.wait_netif_up().await?;
        let ip_info = self.wifi.get_ip_info()?;
        info!("Wifi DHCP info: {:?}", ip_info);
        info!("Setting MDNS hostname");
        self.mdns.set_hostname("burptech_mdns_host")?;
        info!("Setting MDNS instance name");
        self.mdns.set_instance_name("burptech_mdns_instance")?;
        info!("Adding burptech service");
        self.mdns.add_service(Some("burptech_mdns_service"), "_burptech", "_tcp", 1234, &[])?;
        info!("Adding another service");
        self.mdns.add_service(Some("another_mdns_service"), "_another", "_tcp", 1234, &[])?;
        Ok(())
    }
}
