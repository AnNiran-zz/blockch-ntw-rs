/// Implementation of errors used in wire namespace
/// 
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    /// Network message contains TOR address
    ErrorTORAddress,
    /// Address length error
    ErrorAddressLength,

    /// Messages Errors
    ErrorBlockHeaderInvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ErrorTORAddress               => f.write_str(error::Error::description(self)),
            Error::ErrorAddressLength            => f.write_str(error::Error::description(self)),
            Error::ErrorBlockHeaderInvalidLength => f.write_str(error::Error::description(self)),
        }
    }
}

impl error::Error for Error {
    fn reason(&self) -> Option<&error::Error> {
        match *self {
            Error::ErrorTORAddress               => Some(e),
            Error::ErrorAddressLength            => Some(e),
            Error::ErrorBlockHeaderInvalidLength => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::ErrorTORAddress               => "message contains TOR address",
            Error::ErrorAddressLength            => "wrong address length",
            Error::ErrorBlockHeaderInvalidLength => "block header of invalid length",
        }
    }
}