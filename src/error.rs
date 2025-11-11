use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Network(Box<dyn StdError + Send + Sync>),
    Parse(Box<dyn StdError + Send + Sync>),
    DataNotFound(String),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
            Error::DataNotFound(msg) => write!(f, "Data not found: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Network(e) => Some(e.as_ref()),
            Error::Parse(e) => Some(e.as_ref()),
            Error::DataNotFound(_) => None,
            Error::Internal(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(Box::new(err))
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Parse(Box::new(err))
    }
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for Error {
    fn from(err: scraper::error::SelectorErrorKind<'a>) -> Self {
        Error::Internal(format!("CSS selector error: {}", err))
    }
}

impl From<lapin::Error> for Error {
    fn from(err: lapin::Error) -> Self {
        Error::Network(Box::new(err))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Parse(Box::new(err))
    }
}
