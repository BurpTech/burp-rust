use std::error::Error;

pub trait Mdns<E: Error> {
    fn set_hostname(
        &mut self,
        hostname: &str,
    ) -> Result<(), E>;
    fn set_instance_name(
        &mut self,
        instance_name: &str,
    ) -> Result<(), E>;
    fn add_service(
        &mut self,
        instance_name: Option<&str>,
        service_type: &str,
        proto: &str,
        port: u16,
        txt: &[(&str, &str)]
    ) -> Result<(), E>;
}
