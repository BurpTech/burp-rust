use std::error::Error;
use crate::storage::Storage;
use crate::config::read_write::ReadWrite;
use crate::debug::debug_blob::DebugBlob;

pub struct BlobConfig<'a, const N: usize> {
    name: &'a str,
    default: &'a [u8],
    buffer: [u8; N],
    len: usize,
}

impl<'a, const N: usize> BlobConfig<'a, N> {
    pub fn new(name: &'a str, default: &'a [u8]) -> BlobConfig<'a, N> {
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

impl<E: Error, const N: usize> ReadWrite<E> for BlobConfig<'_, N> {
    fn read(&mut self, storage: &dyn Storage<E>) -> Result<(), E> {
        let mut buffer = [0_u8; N];
        let blob_result = storage.get_blob(self.name, &mut buffer);
        blob_result.map(|blob_option| match blob_option {
            None => self.reset(),
            Some(blob) => self.set(blob),
        })
    }

    fn write(&self, storage: &mut dyn Storage<E>) -> Result<(), E> {
        storage.set_blob(self.name, &self.buffer[..self.len])
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use crate::config::blob_config::BlobConfig;
    use crate::mocks::mock_esp_nvs::{MockEspNvs, MockEspNvsValue};
    use crate::config::read_write::ReadWrite;

    #[test]
    fn uses_default_value_when_not_in_storage() {
        MockEspNvs::reset();
        let mock_esp_nvs = MockEspNvs::new();
        let mut blob_config: BlobConfig<100> = BlobConfig::new("name", "default_name".as_bytes());
        blob_config.read(&mock_esp_nvs).unwrap();
        let name = from_utf8(blob_config.get()).unwrap();
        assert_eq!(name, "default_name");
    }

    #[test]
    fn does_read_value_from_storage() {
        MockEspNvs::reset();
        MockEspNvs::insert(String::from("name"), MockEspNvsValue::BlobValue(Vec::from("this is a test")));
        let mock_esp_nvs = MockEspNvs::new();
        let mut blob_config: BlobConfig<100> = BlobConfig::new("name", "default_name".as_bytes());
        blob_config.read(&mock_esp_nvs).unwrap();
        let name = from_utf8(blob_config.get()).unwrap();
        assert_eq!(name, "this is a test");
    }
}
