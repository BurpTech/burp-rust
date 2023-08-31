use std::error::Error;

pub trait Storage<E: Error> {
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a str>, E>;
    fn set_str(&mut self, name: &str, val: &str) -> Result<(), E>;
    fn get_blob<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a [u8]>, E>;
    fn set_blob(&mut self, name: &str, val: &[u8]) -> Result<(), E>;
}
