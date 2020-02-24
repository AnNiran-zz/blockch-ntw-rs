/// A Bitcoin transaction implementation
///
use std::default::Default;
use std::fmt;
use std::io;

use hashes::{sha256d, Hash};

use util::hash::BlockchainHash;
use blockdata::script::Script; // not implemented

/// Reference to a transaction output
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct OutPoint {
    /// Transaction's id
    pub txid: sha256d::Hash,
    /// Index of referenced output in the transaction's vout slice
    pub vout: u32,
}

impl OutPoint {
    /// Create a new OutPoint
    #[inline]
    pub fn new(txid: sha256d::Hash, vout: u32) -> OutPoint {
        OutPoint {
            txid: txid,
            vout: vout,
        }
    }

    /// Create a null OutPount - used for coinbase transactions
    /// because they do not have any previous outputs
    #[inline]
    pub fn null() -> OutPoint {
        OutPoint {
            txid: Default::default(),
            vout: u32::max_value(),
        }
    }

    /// Check if Outpoint is null
    #[inline]
    pub fn is_null(&self) -> bool {
        *self == OutPoint::null()
    }
}

impl Default for OutPoint {
    fn default() -> Self {
        OutPoint::null()
    }
}

impl fmt::Display for OutPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.txid, self.vout)
    }
}

/// Types of errors for OutPoints to be implemented

/// Transaction input implementation
/// defines coins to be consumed
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TxIn {
    // Non-witness
    pub previous_out: OutPoint,
    pub sequence: u32,

    // Witness
    pub value_in: i64,
    pub sig_script: Script,
    // ...
}

// Serialization

/// Transaction output implementation
/// defines source of input
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TxOut {
    pub value_out: u64,
    pub pkscript:  Script,
}

impl Default for TxOut {
    fn default() -> TxOut {
        TxOut { value_out: 0xffffffffffffffffff, sig_script: Script::new() }
    }
}

// Serialization

/// Transaction implementation - holds inputs and outputs
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Transaction {
    /// Protocol version
    /// for example: blr.1.0
    pub version: u32,

    /// block version before which the transaction is valid
    /// or 0 - for immediate validity
    pub lock_time: u32,

    pub input_ent:  Vec<TxIn>,
    pub output_ent: Vec<TxOut>, 
}

impl Transaction {
    /// Computes a normalized transaction id
    pub fn norm_txid(&self) -> sha256d::Hash {
        let clone = Transaction {
            version:    self.version,
            lock_time:  self.lock_time,
            input_ent:  self.input_ent.iter().map(|txin| TxIn { sig_script: Script::new(), .. *txin[]}).collect(),
            output_ent: self.output_ent.clone(),
        };
        clone.blockchain_hash()
    }

    /// Compute a signature hash for an input index with a given sighah flag
    /// ...
    
    /// Verify the transaction is able to spend its output
    /// ...
    
    pub fn is_coin_base(&self) -> bool {
        self.input_ent.len() == 1 && self.input_ent[0].previous_out.is_null()
    }
}