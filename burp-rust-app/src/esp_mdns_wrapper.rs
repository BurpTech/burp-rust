use burp_rust_lib::traits::mdns::Mdns;
use esp_idf_svc::mdns::EspMdns;
use esp_idf_sys::EspError;

pub struct EspMdnsWrapper(pub EspMdns);

impl Mdns for EspMdnsWrapper {
    type Error = EspError;

    fn set_hostname(&mut self, hostname: &str) -> Result<(), Self::Error> {
        self.0.set_hostname(hostname)
    }

    fn set_instance_name(&mut self, instance_name: &str) -> Result<(), Self::Error> {
        self.0.set_instance_name(instance_name)
    }

    fn add_service(&mut self, instance_name: Option<&str>, service_type: &str, proto: &str, port: u16, txt: &[(&str, &str)]) -> Result<(), Self::Error> {
        self.0.add_service(instance_name, service_type, proto, port, txt)
    }
}
