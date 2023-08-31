use crate::storage::Storage;

const NAME_FIELD: &str = "name";
const NAME_MAX_BYTES: usize = 100;
const DEFAULT_NAME: &'static str = "burp_device";

pub struct NameConfig<'a> {
    buffer: [u8; NAME_MAX_BYTES],
    value: Option<&'a str>,
}

impl NameConfig<'_> {
    pub fn from<E>(storage: &dyn Storage<E>) -> Result<NameConfig, E> {
        // let mut buffer = [0_u8; NAME_MAX_BYTES];
        // let value = storage
        //         .get_str(NAME_FIELD, &mut buffer)
        //         .unwrap();
        // NameConfig {
        //     buffer,
        //     value,
        // }
        Ok(NameConfig {
            buffer: [0_u8; NAME_MAX_BYTES],
            value: None,
        })
    }

    pub fn as_str(&self) -> &str {
        self.value.unwrap_or(DEFAULT_NAME)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;
    use crate::config::name_config::{DEFAULT_NAME, NameConfig};
    use crate::mocks::mock_storage::MockStorage;

    #[test]
    fn uses_default_value_when_not_in_storage() -> Result<(), Error>{
        let mock_storage = MockStorage::new();
        let name_config = NameConfig::from(&mock_storage)?;
        assert_eq!(name_config.as_str(), DEFAULT_NAME);
        Ok(())
    }

    #[test]
    fn does_read_value_from_storage() -> Result<(), Error>{
        let mock_storage = MockStorage::from([
            (String::from("name"), String::from("this is a test")),
        ]);
        let name_config = NameConfig::from(&mock_storage)?;
        assert_eq!(name_config.as_str(), "this is a test");
        Ok(())
    }
}
