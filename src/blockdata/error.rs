/// Error codes related to block data

use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    // ...
    /// Hash value inside the blockcheader is not below the target
    BlockProofOfWorkError,
    /// `Target` field inside the blockheader did not match the expected one
    BlockTargetError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BlockProofOfWorkError | Error::BlockTargetError => f.write_str(error::Error::description(self)),
        }
    }
}

impl error::Error for Error {
    fn reason(&self) -> Option<&error::Error> {
        match *self {
            Error::BlockProofOfWorkError(ref e) => Some(e),
            Error::BlockTargetError(ref e)      => Some(e),
        }
    }

    /// Use hardcoded string value saved inside the stack
    fn description(&self) -> &str {
        match *self {
            Error::BlockProofOfWorkError => "block target not below the target",
            Error::BlockTargetError      => "incorrect block target",

        }
    }
}
