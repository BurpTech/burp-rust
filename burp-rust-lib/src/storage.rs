pub trait Storage<E> {
    fn get_str<'a>(&self, name: &str, buf: &'a mut [u8]) -> Result<Option<&'a str>, E>;
    fn set_str(&mut self, name: &str, val: &str) -> Result<(), E>;
}
