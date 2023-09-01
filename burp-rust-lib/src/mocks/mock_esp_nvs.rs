use std::collections::HashMap;
use std::fmt::Debug;
use once_cell::sync::Lazy;

use thiserror::Error;

use crate::storage::Storage;

static mut MOCK_ESP_NVS_KEY_VALUES: Lazy<HashMap<String, MockEspNvsValue>> = Lazy::new(HashMap::new);

pub enum MockEspNvsValue {
    BlobValue(Vec<u8>),
    U8Value(u8),
    I8Value(i8),
    U16Value(u16),
    I16Value(i16),
    U32Value(u32),
    I32Value(i32),
    U64Value(u64),
    I64Value(i64),
}

pub struct MockEspNvs {}

#[derive(Error, Debug)]
pub enum MockEspNvsStorageError {
    #[error("Wrong type: {0}")]
    WrongType(String)
}

impl MockEspNvs {
    pub fn new() -> MockEspNvs {
        MockEspNvs {}
    }

    pub fn reset() {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.clear();
        }
    }

    pub fn insert(key: String, value: MockEspNvsValue) {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(key, value);
        }
    }
}

impl Storage<MockEspNvsStorageError> for MockEspNvs {
    fn get_blob<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::BlobValue(value) => {
                    buf[..value.len()].clone_from_slice(value);
                    Ok(&buf[..value.len()])
                }
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a blob")))
            }).transpose()
        }
    }

    fn set_blob(&mut self, name: &str, val: &[u8]) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::BlobValue(Vec::from(val)));
            Ok(())
        }
    }

    fn get_u8(&self, name: &str) -> Result<Option<u8>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::U8Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a u8")))
            }).transpose()
        }
    }

    fn set_u8(&self, name: &str, val: u8) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::U8Value(val));
            Ok(())
        }
    }

    fn get_i8(&self, name: &str) -> Result<Option<i8>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::I8Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a i8")))
            }).transpose()
        }
    }

    fn set_i8(&self, name: &str, val: i8) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::I8Value(val));
            Ok(())
        }
    }

    fn get_u16(&self, name: &str) -> Result<Option<u16>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::U16Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a u16")))
            }).transpose()
        }
    }

    fn set_u16(&self, name: &str, val: u16) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::U16Value(val));
            Ok(())
        }
    }

    fn get_i16(&self, name: &str) -> Result<Option<i16>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::I16Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a i16")))
            }).transpose()
        }
    }

    fn set_i16(&self, name: &str, val: i16) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::I16Value(val));
            Ok(())
        }
    }

    fn get_u32(&self, name: &str) -> Result<Option<u32>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::U32Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a u32")))
            }).transpose()
        }
    }

    fn set_u32(&self, name: &str, val: u32) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::U32Value(val));
            Ok(())
        }
    }

    fn get_i32(&self, name: &str) -> Result<Option<i32>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::I32Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a i32")))
            }).transpose()
        }
    }

    fn set_i32(&self, name: &str, val: i32) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::I32Value(val));
            Ok(())
        }
    }

    fn get_u64(&self, name: &str) -> Result<Option<u64>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::U64Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a u64")))
            }).transpose()
        }
    }

    fn set_u64(&self, name: &str, val: u64) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::U64Value(val));
            Ok(())
        }
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.get(name).map(|value| match value {
                MockEspNvsValue::I64Value(value) => Ok(*value),
                _ => Err(MockEspNvsStorageError::WrongType(String::from("Not a i64")))
            }).transpose()
        }
    }

    fn set_i64(&self, name: &str, val: i64) -> Result<(), MockEspNvsStorageError> {
        unsafe {
            MOCK_ESP_NVS_KEY_VALUES.insert(String::from(name), MockEspNvsValue::I64Value(val));
            Ok(())
        }
    }
}
