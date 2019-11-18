use std::fmt;
use std::io;
use std::num;

use std::error;

#[derive(Debug)]
pub enum Error {
    InvalidGameInput,
    Logger(flexi_logger::FlexiLoggerError),
    Terminal(crossterm::ErrorKind),
    Io(io::Error),
    BadParse(num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidGameInput => write!(f, "Invalid game input."),
            Error::Logger(ref err) => write!(f, "Logger error: {}", err),
            Error::Terminal(ref err) => write!(f, "Terminal error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::BadParse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidGameInput => "Invalid game input.",
            Error::Logger(ref err) => err.description(),
            Error::Terminal(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::BadParse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::InvalidGameInput => None,
            Error::Logger(ref err) => Some(err),
            Error::Terminal(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::BadParse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::BadParse(err)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Error {
        Error::Terminal(err)
    }
}

impl From<flexi_logger::FlexiLoggerError> for Error {
    fn from(err: flexi_logger::FlexiLoggerError) -> Error {
        Error::Logger(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
