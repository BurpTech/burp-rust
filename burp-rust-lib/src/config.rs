mod name_config;

use std::error::Error;
use crate::config::name_config::NameConfig;
use crate::storage::Storage;

pub struct Config {
    pub name: NameConfig,
}

impl Config {
    pub fn from<E: Error>(storage: &dyn Storage<E>) -> Result<Config, E> {
        let name = NameConfig::from(storage);
        name.map(|name| Config {
            name,
        })
    }
}
