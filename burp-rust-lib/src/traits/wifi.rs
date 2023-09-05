use std::error::Error;
use embedded_svc::ipv4::IpInfo;
use embedded_svc::wifi::{AccessPointInfo, Configuration};

pub trait Wifi {
    type Error: Error;
    fn set_configuration(&mut self, conf: &Configuration) -> Result<(), Self::Error>;
    async fn start(&mut self) -> Result<(), Self::Error>;
    async fn scan(&mut self) -> Result<Vec<AccessPointInfo>, Self::Error>;
    async fn connect(&mut self) -> Result<(), Self::Error>;
    async fn wait_netif_up(&self) -> Result<(), Self::Error>;
    fn get_ip_info(&self) -> Result<IpInfo, Self::Error>;
}
