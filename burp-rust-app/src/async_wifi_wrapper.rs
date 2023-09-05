use burp_rust_lib::traits::wifi::Wifi;
use embedded_svc::ipv4::IpInfo;
use embedded_svc::wifi::{AccessPointInfo, Configuration};
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_sys::EspError;

pub struct AsyncWifiWrapper<'a>(pub AsyncWifi<EspWifi<'a>>);

impl Wifi for AsyncWifiWrapper<'_> {
    type Error = EspError;

    fn set_configuration(&mut self, conf: &Configuration) -> Result<(), Self::Error> {
        self.0.set_configuration(conf)
    }

    async fn start(&mut self) -> Result<(), EspError> {
        self.0.start().await
    }

    async fn scan(&mut self) -> Result<Vec<AccessPointInfo>, EspError> {
        self.0.scan().await
    }

    async fn connect(&mut self) -> Result<(), EspError> {
        self.0.connect().await
    }

    async fn wait_netif_up(&self) -> Result<(), EspError> {
        self.0.wait_netif_up().await
    }

    fn get_ip_info(&self) -> Result<IpInfo, Self::Error> {
        self.0.wifi().sta_netif().get_ip_info()
    }
}
