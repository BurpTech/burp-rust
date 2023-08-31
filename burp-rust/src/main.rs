use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // let esp_nvs_partition = EspDefaultNvsPartition::take()
    //     .unwrap();
    // let esp_nvs = EspNvs::new(
    //     esp_nvs_partition,
    //     "burp",
    //     true
    // ).unwrap();

    info!("Hello, world");

    // let esp_nvs_wrapper = EspNvsWrapper(esp_nvs);

    // let config = Config::new(&esp_nvs_wrapper);
    // info!("{}", config.name.as_str())
}
