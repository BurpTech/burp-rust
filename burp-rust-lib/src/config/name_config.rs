use std::error::Error;
use std::str::{from_utf8, Utf8Error};
use crate::storage::Storage;

const NAME_FIELD: &str = "name";
const NAME_MAX_BYTES: usize = 100;
const DEFAULT_NAME: &'static str = "burp_device";

pub struct NameConfig {
    buffer: Option<[u8; NAME_MAX_BYTES]>,
}

impl NameConfig {
    pub fn from<E: Error>(storage: &dyn Storage<E>) -> Result<NameConfig, E> {
        let mut buffer = [0_u8; NAME_MAX_BYTES];
        let mut config = NameConfig {
            buffer: Some([0_u8; NAME_MAX_BYTES]),
        };
        let blob = storage.get_blob(NAME_FIELD, &mut buffer);
        match blob {
            Ok(blob) => {
                if let Some(value) = blob {
                    config.buffer.unwrap().copy_from_slice(&value);
                } else {
                    config.buffer = None
                }
                Ok(config)
            }
            Err(error) => Err(error),
        }
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        match self.buffer {
            None => Ok(DEFAULT_NAME),
            Some(buffer) => from_utf8(&buffer),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::name_config::{DEFAULT_NAME, NameConfig};
    use crate::mocks::config_error::ConfigError;
    use crate::mocks::mock_storage::{MockStorage, MockStorageError, MockStorageValue};

    #[test]
    fn uses_default_value_when_not_in_storage() -> Result<(), ConfigError<MockStorageError>> {
        let mock_storage = MockStorage::new();
        let name_config = NameConfig::from(&mock_storage).map_err(ConfigError::StorageError)?;
        assert_eq!(name_config.as_str().map_err(ConfigError::Utf8Error)?, DEFAULT_NAME);
        Ok(())
    }

    #[test]
    fn does_read_value_from_storage() -> Result<(), ConfigError<MockStorageError>> {
        let mock_storage = MockStorage::from([
            (String::from("name"), MockStorageValue::StringValue(String::from("this is a test"))),
        ]);
        let name_config = NameConfig::from(&mock_storage).map_err(ConfigError::StorageError)?;
        assert_eq!(name_config.as_str().map_err(ConfigError::Utf8Error)?, "this is a test");
        Ok(())
    }
}
