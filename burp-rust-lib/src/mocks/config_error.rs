use std::error::Error;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError<E: Error> {
    StorageError(E),
    Utf8Error(Utf8Error),
}
