use std::error::Error;

pub trait Mdns {
    type Error: Error;
    fn set_hostname(
        &mut self,
        hostname: &str,
    ) -> Result<(), Self::Error>;
    fn set_instance_name(
        &mut self,
        instance_name: &str,
    ) -> Result<(), Self::Error>;
    fn add_service(
        &mut self,
        instance_name: Option<&str>,
        service_type: &str,
        proto: &str,
        port: u16,
        txt: &[(&str, &str)]
    ) -> Result<(), Self::Error>;
}
