mod blob_config;
pub mod read_write;

use std::error::Error;
use crate::config::blob_config::{BlobConfig};
use crate::storage::Storage;
use crate::config::read_write::ReadWrite;

const NAME_FIELD: &str = "name";
const NAME_MAX_BYTES: usize = 100;
const DEFAULT_NAME: &'static str = "burp_device";

const PASSWORD_FIELD: &str = "password";
const PASSWORD_MAX_BYTES: usize = 100;
const DEFAULT_PASSWORD: &'static str = "let me in!";

pub struct Config<'a> {
    pub name: BlobConfig<'a, NAME_MAX_BYTES>,
    pub password: BlobConfig<'a, PASSWORD_MAX_BYTES>,
}

impl Config<'_> {
    pub fn new<'a>() -> Config<'a> {
        Config {
            name: BlobConfig::new(NAME_FIELD, DEFAULT_NAME.as_bytes()),
            password: BlobConfig::new(PASSWORD_FIELD, DEFAULT_PASSWORD.as_bytes()),
        }
    }
}

impl<E: Error> ReadWrite<E> for Config<'_> {
    fn read(&mut self, storage: &dyn Storage<E>) -> Result<(), E> {
        let fields: [&mut dyn ReadWrite<E>; 2] = [&mut self.name, &mut self.password];
        let iter = fields.map(|field| field.read(storage));
        Result::from_iter(iter).map(|_: ()| ())
    }

    fn write(&self, storage: &mut dyn Storage<E>) -> Result<(), E> {
        let fields: [&dyn ReadWrite<E>; 2] = [&self.name, &self.password];
        let iter = fields.map(|field| field.write(storage));
        Result::from_iter(iter).map(|_: ()| ())
    }
}
