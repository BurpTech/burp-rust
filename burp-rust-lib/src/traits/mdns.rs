use std::error::Error;
use std::time::Duration;

use embedded_svc::ipv4::IpAddr;

#[derive(Debug)]
pub enum Protocol {
    V4,
    V6,
}

#[derive(Debug)]
pub enum Interface {
    STA,
    AP,
    ETH,
}

#[derive(Debug)]
pub struct QueryResult<'a> {
    pub instance_name: &'a Option<String>,
    pub hostname: &'a Option<String>,
    pub port: u16,
    pub txt: &'a Vec<(String, String)>,
    pub addr: &'a Vec<IpAddr>,
    pub interface: Interface,
    pub ip_protocol: Protocol,
}

pub trait Mdns {
    type Error: Error;
    type QueryResult;
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
        txt: &[(&str, &str)],
    ) -> Result<(), Self::Error>;
    fn query_ptr(
        &self,
        service_type: &str,
        proto: &str,
        timeout: Duration,
        max_results: usize,
        results: &mut [Self::QueryResult],
    ) -> Result<usize, Self::Error>;
    fn create_query_results<const N: usize>() -> heapless::Vec<Self::QueryResult, N>;
    fn convert_query_result(query_result: &Self::QueryResult) -> QueryResult;
}
