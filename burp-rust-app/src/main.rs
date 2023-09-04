use std::ffi::CStr;
use std::future::Future;

use burp_rust_lib::config::Config;
use burp_rust_lib::network::Network;
use burp_rust_lib::traits::read_write::ReadWrite;
use edge_executor::SpawnError;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::task::executor::EspExecutor;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mdns::EspMdns;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_sys::{esp_err_to_name, EspError};
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

async fn print_async_error(
    result: impl Future<Output=Result<(), EspError>>,
) {
    let r = result.await;
    if let Err(error) = r {
        let c_str = unsafe { CStr::from_ptr(esp_err_to_name(error.code())) };
        error!("Error encountered: {}", c_str.to_str().unwrap());
    }
}

fn main() -> Result<(), SpawnError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut nvs = init_nvs();
    let config = init_config(&mut nvs);
    let mut wifi = init_async_wifi();
    let mut mdns = init_mdns();
    let mut network = Network::new(&config, &mut wifi, &mut mdns);

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
        EspWifi::new(peripherals.modem, esp_system_event_loop.clone(), None).unwrap(),
        esp_system_event_loop.clone(),
        EspTaskTimerService::new().unwrap(),
    ).unwrap())
}

fn init_config(nvs: &mut EspNvsWrapper<NvsDefault>) -> Config {
    let mut config = Config::new(WIFI_CONFIG.wifi_ssid, WIFI_CONFIG.wifi_psk);
    config.read(nvs).unwrap();
    config
}

fn init_nvs() -> EspNvsWrapper<NvsDefault> {
    let esp_nvs_partition = EspDefaultNvsPartition::take()
        .unwrap();
    let esp_nvs = EspNvs::new(
        esp_nvs_partition,
        "burp",
        true,
    ).unwrap();
    EspNvsWrapper(esp_nvs)
}
