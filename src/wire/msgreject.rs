/// Implementation of rejection message and possible reasons 
/// for the rejection

use std::io;
use consensus::{Encodable, Decodable, ReadExtend};
use consensus::encode;

/// `reject` message codes corresponding to reasons
#[derive(PartialEq, Eq, Clone, copy, Debug)]
pub enum ReasonRejection {
    /// malformed message
    MALFORMED = 0x01;
    /// invalid message - not consistent with active protocol
    INVALID = 0x10;
    /// 
    OBSOLETE = 0x11;
    /// duplicate message
    DUPLICATE = 0x12;
    /// nonstandard message - not consistent with the active protocol
    NONSTANDARD = 0x40;
    /// output below dust limit
    DUST = 0x41;
    /// insufficient fees dedicated in outputs
    FEE = 0x42;
    /// 
    CHECKPOINT = 0x43;
}

impl Encodable for ReasonRejection {
    fn encode<Writer: io::Write>(&self, mut e: W) -> Result<usize, encode::Error> {
        e.write_u8(*self as u8)?;
        Ok(1)
    }
}

impl Decodable for ReasonRejection {
    fn decode<Dec: io::Read>(mut decoder: Dec) -> Result<Self, encode::Error> {
        Ok(match decoder.read_u8()? {
            0x01 => MALFORMED,
            0x10 => INVALID,
            0x11 => OBSOLETE,
            0x12 => DUPLICATE,
            0x40 => NONSTANDARD,
            0x41 => DUST,
            0x42 => FEE,
            0x43 => CHECKPOINT,
            _    => return Err(encode::Error::ErrorParse("unknown rejection code"))
        })
    }
}

pub struct RejectMessage {
    /// the message type that has been rejected
    pub message: String,
    /// rejection code
    pub code: ReasonRejection,
    /// rejection reason as a string
    pub reason: String,
    /// rejected item hash
    pub hash: sha256d::Hash
}

impl RejectMessage -> RejectMessage {
    pub fn new_reject_msg(
        message: String, 
        code:    ReasonRejection, 
        reason:  String, 
        hash:    sha256d::Hash
    ) {
        RejectMessage {
            message: message,
            code:    code,
            reason:  reason,
            hash:    hash
        }
    }
}

consensus_encoding!(RejectMessage, message, code, reason, hash);