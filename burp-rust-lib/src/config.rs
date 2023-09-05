use std::sync::{Arc, Mutex};

use crate::config::blob_config::BlobConfig;
use crate::traits::read_write::ReadWrite;
use crate::traits::storage::Storage;

mod blob_config;

const SSID_FIELD: &str = "ssid";
const SSID_MAX_BYTES: usize = 32;

const PSK_FIELD: &str = "psk";
const PSK_MAX_BYTES: usize = 64;

pub struct Config<'a, S> {
    pub ssid: BlobConfig<'a, S, SSID_MAX_BYTES>,
    pub psk: BlobConfig<'a, S, PSK_MAX_BYTES>,
}

impl<S: Storage> Config<'_, S> {
    pub fn new<'a>(storage: Arc<Mutex<S>>, default_ssid: &'a str, default_psk: &'a str) -> Config<'a, S> {
        Config {
            ssid: BlobConfig::new(storage.clone(), SSID_FIELD, default_ssid.as_bytes()),
            psk: BlobConfig::new(storage.clone(), PSK_FIELD, default_psk.as_bytes()),
        }
    }
}

impl<S: Storage> ReadWrite for Config<'_, S> {
    type Error = S::Error;

    fn read(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 2] = [
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.read());
        Result::from_iter(iter).map(|_: ()| ())
    }

    fn write(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 2] = [
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.write());
        Result::from_iter(iter).map(|_: ()| ())
    }
}
