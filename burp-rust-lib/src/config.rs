use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use const_hex::encode_to_slice_upper;
use log::info;

use crate::config::blob_config::BlobConfig;
use crate::traits::read_write::ReadWrite;
use crate::traits::storage::Storage;

mod blob_config;

const NAME_FIELD: &str = "name";
const NAME_MAX_BYTES: usize = 100;
const DEFAULT_NAME_PREFIX: &'static str = "burp-";
const DEFAULT_NAME_PREFIX_LEN: usize = DEFAULT_NAME_PREFIX.as_bytes().len();
const DEFAULT_NAME_SUFFIX_LENGTH: usize = 12;
const DEFAULT_NAME_LENGTH: usize = DEFAULT_NAME_PREFIX_LEN + DEFAULT_NAME_SUFFIX_LENGTH;
static mut DEFAULT_NAME: [u8; DEFAULT_NAME_LENGTH] = [0_u8; DEFAULT_NAME_LENGTH];

const SSID_FIELD: &str = "ssid";
const SSID_MAX_BYTES: usize = 32;

const PSK_FIELD: &str = "psk";
const PSK_MAX_BYTES: usize = 64;

pub struct Config<'a, S> {
    pub name: BlobConfig<'a, S, NAME_MAX_BYTES>,
    pub ssid: BlobConfig<'a, S, SSID_MAX_BYTES>,
    pub psk: BlobConfig<'a, S, PSK_MAX_BYTES>,
}

fn create_default_name(base_mac_address: &[u8; 6]) {
    unsafe {
        DEFAULT_NAME[..DEFAULT_NAME_PREFIX_LEN].copy_from_slice(DEFAULT_NAME_PREFIX.as_bytes());
        encode_to_slice_upper(base_mac_address, &mut DEFAULT_NAME[DEFAULT_NAME_PREFIX_LEN..]).unwrap();
        info!("default_name is: {}", from_utf8(&DEFAULT_NAME).unwrap());
    }
}

impl<S: Storage> Config<'_, S> {
    pub fn new<'a>(storage: Arc<Mutex<S>>, base_mac_address: &[u8; 6], default_ssid: &'a str, default_psk: &'a str) -> Config<'a, S> {
        create_default_name(base_mac_address);
        Config {
            name: BlobConfig::new(storage.clone(), NAME_FIELD, unsafe { &DEFAULT_NAME }),
            ssid: BlobConfig::new(storage.clone(), SSID_FIELD, default_ssid.as_bytes()),
            psk: BlobConfig::new(storage.clone(), PSK_FIELD, default_psk.as_bytes()),
        }
    }
}

impl<S: Storage> ReadWrite for Config<'_, S> {
    type Error = S::Error;

    fn read(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 3] = [
            &mut self.name,
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.read());
        Result::from_iter(iter).map(|_: ()| ())
    }

    fn write(&mut self) -> Result<(), Self::Error> {
        let fields: [&mut dyn ReadWrite<Error=Self::Error>; 3] = [
            &mut self.name,
            &mut self.ssid,
            &mut self.psk,
        ];
        let iter = fields.map(|field| field.write());
        Result::from_iter(iter).map(|_: ()| ())
    }
}
