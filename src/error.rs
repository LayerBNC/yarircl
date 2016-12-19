use std::{fmt, error};

#[derive(Debug)]
pub enum Error {
    InvalidHostmaskString
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidHostmaskString => write!(f, "invalid hostmask string (hostmask should have the following format: nick!user@host.domain)")
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidHostmaskString => "invalid hostmask string"
        }
    }
}