use burp_rust_lib::traits::storage::Storage;
use esp_idf_svc::nvs::EspNvs;
use esp_idf_sys::EspError;

pub struct EspNvsWrapper<T: esp_idf_svc::nvs::NvsPartitionId>(pub EspNvs<T>);

impl<T: esp_idf_svc::nvs::NvsPartitionId> Storage for EspNvsWrapper<T> {
    type Error = EspError;

    fn get_blob<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, Self::Error> {
        self.0.get_blob(name, buf)
    }

    fn set_blob(&mut self, name: &str, val: &[u8]) -> Result<(), Self::Error> {
        self.0.set_blob(name, val)
    }

    fn get_u8(&self, name: &str) -> Result<Option<u8>, Self::Error> {
        self.0.get_u8(name)
    }

    fn set_u8(&self, name: &str, val: u8) -> Result<(), Self::Error> {
        self.0.set_u8(name, val)
    }

    fn get_i8(&self, name: &str) -> Result<Option<i8>, Self::Error> {
        self.0.get_i8(name)
    }

    fn set_i8(&self, name: &str, val: i8) -> Result<(), Self::Error> {
        self.0.set_i8(name, val)
    }

    fn get_u16(&self, name: &str) -> Result<Option<u16>, Self::Error> {
        self.0.get_u16(name)
    }

    fn set_u16(&self, name: &str, val: u16) -> Result<(), Self::Error> {
        self.0.set_u16(name, val)
    }

    fn get_i16(&self, name: &str) -> Result<Option<i16>, Self::Error> {
        self.0.get_i16(name)
    }

    fn set_i16(&self, name: &str, val: i16) -> Result<(), Self::Error> {
        self.0.set_i16(name, val)
    }

    fn get_u32(&self, name: &str) -> Result<Option<u32>, Self::Error> {
        self.0.get_u32(name)
    }

    fn set_u32(&self, name: &str, val: u32) -> Result<(), Self::Error> {
        self.0.set_u32(name, val)
    }

    fn get_i32(&self, name: &str) -> Result<Option<i32>, Self::Error> {
        self.0.get_i32(name)
    }

    fn set_i32(&self, name: &str, val: i32) -> Result<(), Self::Error> {
        self.0.set_i32(name, val)
    }

    fn get_u64(&self, name: &str) -> Result<Option<u64>, Self::Error> {
        self.0.get_u64(name)
    }

    fn set_u64(&self, name: &str, val: u64) -> Result<(), Self::Error> {
        self.0.set_u64(name, val)
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, Self::Error> {
        self.0.get_i64(name)
    }

    fn set_i64(&self, name: &str, val: i64) -> Result<(), Self::Error> {
        self.0.set_i64(name, val)
    }
}
