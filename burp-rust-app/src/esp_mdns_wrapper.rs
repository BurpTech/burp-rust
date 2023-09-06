use std::time::Duration;

use burp_rust_lib::traits::mdns::{Interface, Mdns, Protocol, QueryResult};
use esp_idf_svc::mdns::{EspMdns, Interface as EspInterface, Protocol as EspProtocol, QueryResult as EspQueryResult};
use esp_idf_sys::EspError;

pub struct EspMdnsWrapper(pub EspMdns);

fn convert_query_result_interface(interface: &EspInterface) -> Interface {
    match interface {
        EspInterface::STA => Interface::STA,
        EspInterface::AP => Interface::AP,
        EspInterface::ETH => Interface::ETH,
    }
}

fn convert_query_result_ip_protocol(ip_protocol: &EspProtocol) -> Protocol {
    match ip_protocol {
        EspProtocol::V4 => Protocol::V4,
        EspProtocol::V6 => Protocol::V6,
    }
}

impl Mdns for EspMdnsWrapper {
    type Error = EspError;
    type QueryResult = EspQueryResult;

    fn set_hostname(&mut self, hostname: &str) -> Result<(), Self::Error> {
        self.0.set_hostname(hostname)
    }

    fn set_instance_name(&mut self, instance_name: &str) -> Result<(), Self::Error> {
        self.0.set_instance_name(instance_name)
    }

    fn add_service(&mut self, instance_name: Option<&str>, service_type: &str, proto: &str, port: u16, txt: &[(&str, &str)]) -> Result<(), Self::Error> {
        self.0.add_service(instance_name, service_type, proto, port, txt)
    }

    fn query_ptr(&self, service_type: &str, proto: &str, timeout: Duration, max_results: usize, results: &mut [Self::QueryResult]) -> Result<usize, Self::Error> {
        self.0.query_ptr(service_type, proto, timeout, max_results, results)
    }

    fn create_query_results<const N: usize>() -> heapless::Vec<Self::QueryResult, N> {
        heapless::Vec::new()
    }

    fn convert_query_result(query_result: &Self::QueryResult) -> QueryResult {
        QueryResult {
            instance_name: &query_result.instance_name,
            hostname: &query_result.hostname,
            port: query_result.port,
            txt: &query_result.txt,
            addr: &query_result.addr,
            interface: convert_query_result_interface(&query_result.interface),
            ip_protocol: convert_query_result_ip_protocol(&query_result.ip_protocol),
        }
    }
}
