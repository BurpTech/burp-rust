use std::error::Error;
use async_trait::async_trait;
use embedded_svc::ipv4::IpInfo;
use embedded_svc::wifi::{AccessPointInfo, Configuration};

#[async_trait(?Send)]
pub trait Wifi<E: Error> {
    fn set_configuration(&mut self, conf: &Configuration) -> Result<(), E>;
    async fn start(&mut self) -> Result<(), E>;
    async fn scan(&mut self) -> Result<Vec<AccessPointInfo>, E>;
    async fn connect(&mut self) -> Result<(), E>;
    async fn wait_netif_up(&self) -> Result<(), E>;
    fn get_ip_info(&self) -> Result<IpInfo, E>;
}
