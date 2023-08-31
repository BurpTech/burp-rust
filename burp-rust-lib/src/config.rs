mod name_config;

use crate::config::name_config::NameConfig;
use crate::storage::Storage;

pub struct Config<'a> {
    pub name: NameConfig<'a>,
}

impl Config<'_> {
    pub fn from<E>(storage: &dyn Storage<E>) -> Result<Config, E> {
        let name = NameConfig::from(storage);
        name.map(|name| Config {
            name,
        })
    }
}
