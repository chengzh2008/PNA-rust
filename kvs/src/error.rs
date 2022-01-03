use std::io;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    IoErr(io::Error),
    #[fail(display = "{}", _0)]
    SerdeErr(serde_json::Error),
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "Unexpected command")]
    UnexpectedCommandErr,
}

pub type Result<T> = std::result::Result<T, KvsError>;
