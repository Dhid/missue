use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

#[derive(Fail, Debug)]
pub enum MissueError {
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,

    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for MissueError {
    fn from(err: io::Error) -> MissueError {
        MissueError::Io(err)
    }
}

impl From<serde_json::Error> for MissueError {
    fn from(err: serde_json::Error) -> MissueError {
        MissueError::Serde(err)
    }
}

impl From<FromUtf8Error> for MissueError {
    fn from(err: FromUtf8Error) -> MissueError {
        MissueError::Utf8(err)
    }
}

pub type Result<T> = std::result::Result<T, MissueError>;
