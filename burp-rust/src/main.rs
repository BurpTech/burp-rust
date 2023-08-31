use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::EspError;
use log::*;
use burp_rust_lib::config::Config;
use burp_rust_lib::storage::Storage;

struct EspNvsWrapper<T: esp_idf_svc::nvs::NvsPartitionId>(EspNvs<T>);

impl<T: esp_idf_svc::nvs::NvsPartitionId> Storage<EspError> for EspNvsWrapper<T> {
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a str>, EspError> {
        self.0.get_str(name, buf)
    }

    fn set_str(&mut self, name: &str, val: &str) -> Result<(), EspError> {
        self.0.set_str(name, val)
    }
}

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

    let esp_nvs_wrapper = EspNvsWrapper(esp_nvs);

    let config = Config::from(&esp_nvs_wrapper).unwrap();
    info!("{}", config.name.as_str())
}
