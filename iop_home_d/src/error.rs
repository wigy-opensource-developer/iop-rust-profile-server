use std::net;
use std::io;
use std::fmt;
use std::error::Error as StdError;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
    Addr(net::AddrParseError), 
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Addr(ref err) => write!(f, "Address parse error: {}", err),
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Addr(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Addr(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
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

#[must_use]
pub type Result<T> = StdResult<T, Error>;
