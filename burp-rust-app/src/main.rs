use std::str::from_utf8;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use log::*;
use burp_rust_app::esp_nvs_wrapper::EspNvsWrapper;
use burp_rust_lib::config::Config;
use burp_rust_lib::config::read_write::ReadWrite;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let esp_nvs_partition = EspDefaultNvsPartition::take()
        .unwrap();
    let esp_nvs = EspNvs::new(
        esp_nvs_partition,
        "burp",
        true
    ).unwrap();

    let mut esp_nvs_wrapper = EspNvsWrapper(esp_nvs);

    let mut config = Config::new();
    config.read(&mut esp_nvs_wrapper).unwrap();
    info!("before set: name: {}", from_utf8(config.name.get()).unwrap());
    info!("before set: password: {}", from_utf8(config.password.get()).unwrap());
    config.name.set("banana".as_bytes());
    config.password.set("oh no you don't".as_bytes());
    info!("after set: name: {}", from_utf8(config.name.get()).unwrap());
    info!("after set: password: {}", from_utf8(config.password.get()).unwrap());
    config.write(&mut esp_nvs_wrapper).unwrap();
    info!("after write: name: {}", from_utf8(config.name.get()).unwrap());
    info!("after write: password: {}", from_utf8(config.password.get()).unwrap());
}
