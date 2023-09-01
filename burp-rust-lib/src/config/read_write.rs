use std::error::Error;
use crate::storage::Storage;

pub trait ReadWrite<E: Error> {
    fn read(&mut self, storage: &dyn Storage<E>) -> Result<(), E>;
    fn write(&self, storage: &mut dyn Storage<E>) -> Result<(), E>;
}
