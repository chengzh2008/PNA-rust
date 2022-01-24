use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

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
    #[fail(display = "Sled err")]
    SledErr(sled::Error),
    #[fail(display = "utf8 conversion error")]
    Utf8Err,
}

pub type Result<T> = std::result::Result<T, KvsError>;

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::IoErr(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::SerdeErr(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::SledErr(err)
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(err: FromUtf8Error) -> KvsError {
        KvsError::Utf8Err
    }
}
