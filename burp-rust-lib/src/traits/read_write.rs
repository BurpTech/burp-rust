use std::error::Error;

pub trait ReadWrite {
    type Error: Error;
    fn read(&mut self) -> Result<(), Self::Error>;
    fn write(&mut self) -> Result<(), Self::Error>;
}
