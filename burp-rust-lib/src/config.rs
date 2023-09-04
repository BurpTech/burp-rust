mod blob_config;

use std::error::Error;
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

pub struct Config<'a> {
    pub name: BlobConfig<'a, NAME_MAX_BYTES>,
    pub password: BlobConfig<'a, PASSWORD_MAX_BYTES>,
    pub ssid: BlobConfig<'a, SSID_MAX_BYTES>,
    pub psk: BlobConfig<'a, PSK_MAX_BYTES>,
}

impl Config<'_> {
    pub fn new<'a>(default_ssid: &'a str, default_psk: &'a str) -> Config<'a> {
        Config {
            name: BlobConfig::new(NAME_FIELD, DEFAULT_NAME.as_bytes()),
            password: BlobConfig::new(PASSWORD_FIELD, DEFAULT_PASSWORD.as_bytes()),
            ssid: BlobConfig::new(SSID_FIELD, default_ssid.as_bytes()),
            psk: BlobConfig::new(PSK_FIELD, default_psk.as_bytes()),
        }
    }
}

impl<E: Error> ReadWrite<E> for Config<'_> {
    fn read(&mut self, storage: &dyn Storage<E>) -> Result<(), E> {
        let fields: [&mut dyn ReadWrite<E>; 4] = [
            &mut self.name,
            &mut self.password,
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.read(storage));
        Result::from_iter(iter).map(|_: ()| ())
    }

    fn write(&self, storage: &mut dyn Storage<E>) -> Result<(), E> {
        let fields: [&dyn ReadWrite<E>; 4] = [
            &self.name,
            &self.password,
            &self.ssid,
            &self.psk,
        ];
        let iter = fields.map(|field| field.write(storage));
        Result::from_iter(iter).map(|_: ()| ())
    }
}
