use std::collections::HashMap;
use std::fmt::{Debug};
use std::str::from_utf8_unchecked;
use thiserror::Error;
use crate::storage::Storage;

pub enum MockStorageValue {
    StringValue(String),
    BlobValue(Vec<u8>),
}

pub struct MockStorage {
    key_values: HashMap<String, MockStorageValue>,
}

#[derive(Error, Debug)]
pub enum MockStorageError {
    #[error("Wrong type: {0}")]
    WrongType(String)
}

impl MockStorage {
    pub fn new() -> MockStorage {
        MockStorage {
            key_values: HashMap::new(),
        }
    }
}

impl<const N: usize> From<[(String, MockStorageValue); N]> for MockStorage {
    fn from(value: [(String, MockStorageValue); N]) -> Self {
        MockStorage {
            key_values: HashMap::from(value),
        }
    }
}

impl Storage<MockStorageError> for MockStorage {
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a str>, MockStorageError> {
        let bytes = self
            .key_values
            .get(name)
            .map(|s| match s {
                MockStorageValue::StringValue(value) => Ok(&value.as_bytes()[..buf.len()]),
                _ => Err(MockStorageError::WrongType(String::from("Not a String")))
            });
        if let Some(bytes) = bytes {
            bytes.map(|bytes| {
                buf.clone_from_slice(bytes);
                unsafe {
                    Some(from_utf8_unchecked(buf))
                }
            })
        } else {
            Ok(None)
        }
    }

    fn set_str(&mut self, name: &str, val: &str) -> Result<(), MockStorageError> {
        self.key_values.insert(String::from(name), MockStorageValue::StringValue(String::from(val)));
        Ok(())
    }

    fn get_blob<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, MockStorageError> {
        let bytes = self
            .key_values
            .get(name)
            .map(|s| match s {
                MockStorageValue::BlobValue(value) => Ok(&value[..buf.len()]),
                _ => Err(MockStorageError::WrongType(String::from("Not a Blob")))
            });
        if let Some(bytes) = bytes {
            bytes.map(|bytes| {
                buf.clone_from_slice(bytes);
                Some(&buf[..buf.len()])
            })
        } else {
            Ok(None)
        }
    }

    fn set_blob(&mut self, name: &str, val: &[u8]) -> Result<(), MockStorageError> {
        self
            .key_values
            .insert(String::from(name), MockStorageValue::BlobValue(Vec::from(val)));
        Ok(())
    }
}
