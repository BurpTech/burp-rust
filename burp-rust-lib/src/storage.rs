use std::error::Error;

pub trait Storage<E: Error> {
    fn get_blob<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, E>;
    fn set_blob(&mut self, name: &str, val: &[u8]) -> Result<(), E>;
    fn get_u8(&self, name: &str) -> Result<Option<u8>, E>;
    fn set_u8(&self, name: &str, val: u8) -> Result<(), E>;
    fn get_i8(&self, name: &str) -> Result<Option<i8>, E>;
    fn set_i8(&self, name: &str, val: i8) -> Result<(), E>;
    fn get_u16(&self, name: &str) -> Result<Option<u16>, E>;
    fn set_u16(&self, name: &str, val: u16) -> Result<(), E>;
    fn get_i16(&self, name: &str) -> Result<Option<i16>, E>;
    fn set_i16(&self, name: &str, val: i16) -> Result<(), E>;
    fn get_u32(&self, name: &str) -> Result<Option<u32>, E>;
    fn set_u32(&self, name: &str, val: u32) -> Result<(), E>;
    fn get_i32(&self, name: &str) -> Result<Option<i32>, E>;
    fn set_i32(&self, name: &str, val: i32) -> Result<(), E>;
    fn get_u64(&self, name: &str) -> Result<Option<u64>, E>;
    fn set_u64(&self, name: &str, val: u64) -> Result<(), E>;
    fn get_i64(&self, name: &str) -> Result<Option<i64>, E>;
    fn set_i64(&self, name: &str, val: i64) -> Result<(), E>;
}
