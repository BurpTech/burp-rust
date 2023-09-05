use std::ffi::CStr;
use std::future::Future;
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};

use burp_rust_lib::config::Config;
use burp_rust_lib::name::{get_name, init_name};
use burp_rust_lib::network::{Network, NetworkError};
use burp_rust_lib::traits::read_write::ReadWrite;
use edge_executor::SpawnError;
use embedded_svc::ipv4::{ClientConfiguration, Configuration, DHCPClientSettings};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::task::executor::EspExecutor;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mdns::EspMdns;
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi, WifiDriver};
use esp_idf_sys::{esp, esp_base_mac_addr_get, esp_err_to_name, EspError};
use log::*;

use burp_rust_app::async_wifi_wrapper::AsyncWifiWrapper;
use burp_rust_app::esp_mdns_wrapper::EspMdnsWrapper;
use burp_rust_app::esp_nvs_wrapper::EspNvsWrapper;

#[toml_cfg::toml_config]
pub struct WifiConfig {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn print_utf8_error(utf8_error: Utf8Error) {
    error!("UTF-8 Error encountered: {}", utf8_error);
}

fn print_esp_error(esp_error: EspError) {
    let c_str = unsafe { CStr::from_ptr(esp_err_to_name(esp_error.code())) };
    error!("ESP Error encountered: {}", c_str.to_str().unwrap());
}

async fn print_async_error(
    result: impl Future<Output=Result<(), NetworkError<EspError, EspError>>>,
) {
    let r = result.await;
    if let Err(error) = r {
        match error {
            NetworkError::Utf8Error(utf8_error) => print_utf8_error(utf8_error),
            NetworkError::WifiError(esp_error) => print_esp_error(esp_error),
            NetworkError::MdnsError(esp_error) => print_esp_error(esp_error),
        }
    }
}

fn main() -> Result<(), SpawnError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let base_mac_address = get_base_mac_address().unwrap();
    init_name(&base_mac_address);

    let nvs = init_nvs();
    let config = init_config(nvs.clone());
    let wifi = init_async_wifi();
    let mdns = init_mdns();
    let mut network = Network::new(config.clone(), wifi, mdns);

    let executor = EspExecutor::new();
    let mut tasks = heapless::Vec::<_, 1>::new();
    executor.spawn_local_collect(print_async_error(network.start()), &mut tasks)?;
    executor.run_tasks(|| true, tasks);
    println!("Finished");
    Ok(())
}

fn init_mdns() -> EspMdnsWrapper {
    EspMdnsWrapper(EspMdns::take().unwrap())
}

fn init_async_wifi() -> AsyncWifiWrapper<'static> {
    let peripherals = Peripherals::take().unwrap();
    let esp_system_event_loop = EspSystemEventLoop::take().unwrap();
    AsyncWifiWrapper(AsyncWifi::wrap(
        EspWifi::wrap_all(
            WifiDriver::new(peripherals.modem, esp_system_event_loop.clone(), None).unwrap(),
            EspNetif::new_with_conf(&NetifConfiguration {
                key: "WIFI_STA".into(),
                description: "sta".into(),
                route_priority: 100,
                ip_configuration: Configuration::Client(ClientConfiguration::DHCP(DHCPClientSettings {
                    hostname: Some(get_name().into()),
                })),
                stack: NetifStack::Sta,
                custom_mac: None,
            }).unwrap(),
            EspNetif::new_with_conf(&NetifConfiguration::eth_default_router()).unwrap(),
        ).unwrap(),
        esp_system_event_loop.clone(),
        EspTaskTimerService::new().unwrap(),
    ).unwrap())
}

fn init_config(nvs: Arc<Mutex<EspNvsWrapper<NvsDefault>>>) -> Arc<Mutex<Config<'static, EspNvsWrapper<NvsDefault>>>> {
    let mut config = Config::new(nvs, WIFI_CONFIG.wifi_ssid, WIFI_CONFIG.wifi_psk);
    config.read().unwrap();
    Arc::new(Mutex::new(config))
}

fn init_nvs() -> Arc<Mutex<EspNvsWrapper<NvsDefault>>> {
    let esp_nvs_partition = EspDefaultNvsPartition::take()
        .unwrap();
    let esp_nvs = EspNvs::new(
        esp_nvs_partition,
        "burptech",
        true,
    ).unwrap();
    Arc::new(Mutex::new(EspNvsWrapper(esp_nvs)))
}

fn get_base_mac_address() -> Result<[u8; 6], EspError> {
    let mut mac = [0_u8; 6];
    esp!(unsafe { esp_base_mac_addr_get(mac.as_mut_ptr()) })?;
    Ok(mac)
}
