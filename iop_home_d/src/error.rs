use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::net;
use std::result::Result as StdResult;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    Addr(net::AddrParseError), 
    Io(io::Error),
    Utf8(Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Addr(ref err) => write!(f, "Address parse error: {}", err),
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::Utf8(ref err) => write!(f, "UTF8 error: {}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Addr(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Utf8(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Addr(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
        }
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::Addr(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

#[must_use]
pub type Result<T> = StdResult<T, Error>;
