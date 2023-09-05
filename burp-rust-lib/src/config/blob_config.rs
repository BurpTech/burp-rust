use std::sync::{Arc, Mutex};
use crate::traits::read_write::ReadWrite;
use crate::debug::debug_blob::DebugBlob;
use crate::traits::storage::Storage;

pub struct BlobConfig<'a, S, const N: usize> {
    storage: Arc<Mutex<S>>,
    name: &'a str,
    default: &'a [u8],
    buffer: [u8; N],
    len: usize,
}

impl<'a, S: Storage, const N: usize> BlobConfig<'a, S, N> {
    pub fn new(storage: Arc<Mutex<S>>, name: &'a str, default: &'a [u8]) -> BlobConfig<'a, S, N> {
        let len = default.len();
        assert!(
            len <= N,
            "Default value [{}] is too large for field [{}], max size in bytes is {}, given value is {} bytes",
            DebugBlob::new(default),
            name,
            N,
            len,
        );
        BlobConfig {
            storage,
            default,
            name,
            buffer: [0_u8; N],
            len: 0,
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.buffer[..self.len]
    }

    pub fn set(&mut self, blob: &[u8]) {
        let len = blob.len();
        assert!(
            len <= N,
            "Value [{}] is too large for field [{}], max size in bytes is {}, given value is {} bytes",
            DebugBlob::new(blob),
            self.name,
            N,
            len,
        );
        self.len = len;
        self.buffer[..len].copy_from_slice(blob);
    }

    pub fn reset(&mut self) {
        self.set(self.default);
    }

    pub fn max_bytes(&self) -> usize {
        N
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl<S: Storage, const N: usize> ReadWrite for BlobConfig<'_, S, N> {
    type Error = S::Error;

    fn read(&mut self) -> Result<(), Self::Error> {
        let mut buffer = [0_u8; N];
        let blob_result = self.storage.lock().unwrap().get_blob(self.name, &mut buffer);
        blob_result.map(|blob_option| match blob_option {
            None => self.reset(),
            Some(blob) => self.set(blob),
        })
    }

    fn write(&mut self) -> Result<(), Self::Error> {
        self.storage.lock().unwrap().set_blob(self.name, &self.buffer[..self.len])
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use std::sync::{Arc, Mutex};
    use crate::config::blob_config::BlobConfig;
    use crate::mocks::mock_esp_nvs::{MockEspNvs, MockEspNvsValue};
    use crate::traits::read_write::ReadWrite;

    #[test]
    fn uses_default_value_when_not_in_storage() {
        let mock_esp_nvs = Arc::new(Mutex::new(MockEspNvs::from([])));
        let mut blob_config: BlobConfig<MockEspNvs, 100> = BlobConfig::new(
            mock_esp_nvs.clone(),
            "name",
            "default_name".as_bytes()
        );
        blob_config.read().unwrap();
        let name = from_utf8(blob_config.get()).unwrap();
        assert_eq!(name, "default_name");
    }

    #[test]
    fn does_read_value_from_storage() {
        let mock_esp_nvs = Arc::new(Mutex::new(MockEspNvs::from([
            (String::from("name"), MockEspNvsValue::BlobValue(Vec::from("this is a test"))),
        ])));
        let mut blob_config: BlobConfig<MockEspNvs, 100> = BlobConfig::new(
            mock_esp_nvs.clone(),
            "name",
            "default_name".as_bytes()
        );
        blob_config.read().unwrap();
        let name = from_utf8(blob_config.get()).unwrap();
        assert_eq!(name, "this is a test");
    }
}
