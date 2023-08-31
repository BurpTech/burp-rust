use std::collections::HashMap;
use std::fmt::Error;
use std::str::from_utf8_unchecked;
use crate::storage::Storage;

pub struct MockStorage {
    map: HashMap<String, String>,
}

impl MockStorage {
    pub fn new() -> MockStorage {
        MockStorage {
            map: HashMap::new(),
        }
    }
}

impl<const N: usize> From<[(String, String); N]> for MockStorage {
    fn from(value: [(String, String); N]) -> Self {
        MockStorage {
            map: HashMap::from(value),
        }
    }
}

impl Storage<Error> for MockStorage {
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a str>, Error> {
        let bytes = self.map.get(name).map(|s| &s.as_bytes()[..buf.len()]);
        if let Some(bytes) = bytes {
            buf.clone_from_slice(bytes);
            unsafe {
                Ok(Some(from_utf8_unchecked(buf)))
            }
        } else {
            Ok(None)
        }
    }

    fn set_str(&mut self, name: &str, val: &str) -> Result<(), Error> {
        self.map.insert(String::from(name), String::from(val));
        Ok(())
    }
}
