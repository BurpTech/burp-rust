mod blob_config;

use std::sync::{Arc, Mutex};
use crate::config::blob_config::BlobConfig;
use crate::traits::read_write::ReadWrite;
use crate::traits::storage::Storage;

const NAME_FIELD: &str = "name";
const NAME_MAX_BYTES: usize = 100;
const DEFAULT_NAME: &'static str = "burp_device";

const PASSWORD_FIELD: &str = "password";
const PASSWORD_MAX_BYTES: usize = 100;
const DEFAULT_PASSWORD: &'static str = "let me in!";

const SSID_FIELD: &str = "ssid";
const SSID_MAX_BYTES: usize = 100;

const PSK_FIELD: &str = "psk";
const PSK_MAX_BYTES: usize = 100;

pub struct Config<'a, S> {
    pub name: BlobConfig<'a, S, NAME_MAX_BYTES>,
    pub password: BlobConfig<'a, S, PASSWORD_MAX_BYTES>,
    pub ssid: BlobConfig<'a, S, SSID_MAX_BYTES>,
    pub psk: BlobConfig<'a, S, PSK_MAX_BYTES>,
}

impl<S: Storage> Config<'_, S> {
    pub fn new<'a>(storage: Arc<Mutex<S>>, default_ssid: &'a str, default_psk: &'a str) -> Config<'a, S> {
        Config {
            name: BlobConfig::new(storage.clone(), NAME_FIELD, DEFAULT_NAME.as_bytes()),
            password: BlobConfig::new(storage.clone(), PASSWORD_FIELD, DEFAULT_PASSWORD.as_bytes()),
            ssid: BlobConfig::new(storage.clone(), SSID_FIELD, default_ssid.as_bytes()),
            psk: BlobConfig::new(storage.clone(), PSK_FIELD, default_psk.as_bytes()),
        }
    }
}

impl<S: Storage> ReadWrite for Config<'_, S> {
    type Error = S::Error;

    fn read(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 4] = [
            &mut self.name,
            &mut self.password,
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.read());
        Result::from_iter(iter).map(|_: ()| ())
    }

    fn write(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 4] = [
            &mut self.name,
            &mut self.password,
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.write());
        Result::from_iter(iter).map(|_: ()| ())
    }
}
