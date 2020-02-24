// msgGetHeader
// msgHeader

// msgGetBlock
// msgBlock

/// Implementations for messages related to block data
/// MessageGetHeader
/// MessageHeader
/// MessageGetBlocks
/// MessageBlock
/// 

use std::{io, iter};
use hashes::sha256d;
use util::constants::{PROTOCOL_VERSION, MAX_BLOCK_HEADERS_PER_MSG};

use blockdata::block::{BlockHeader, Block};

/// `getheaders` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GetHeadersMessage {
    /// Protocol version
    pub protocol: u32

    /// List of hashes from newest to oldest (to block 1 is possible)
    /// that the remote peer with reply with - it will send the data
    /// according to the longest chain it sees
    pub block_location_hashes: Vec<sha256d::Hash>,

    /// The block hash to stop at; if set to zero - send all blocks
    pub stop_hash: sha256d::Hash
}

impl GetHeadersMessage {
    pub fn new_get_headers_msg(loc_hashes: Vec<sha256d::Hash>, stop_hash: sha256d:Hash) -> GetHeadersMessage {
        GetHeadersMessage {
            protocol:              constants::PROTOCOL_VERSION,
            block_location_hashes: loc_hashes,
            stop_hash:             stop_hash
        }
    }
}

consensus_encoding!(GetHeadersMessage, protocol, block_location_hashes, stop_hash);

/// `sendheaders` message
#[derive(PartialEq, Clone, Debug)]
pub struct SendHeadersMessage {
    /// Protocol version
    pub protocol: u32,
 
    /// List of headers hashes from newest to oldest form the 
    /// sender viewpoint
    pub headers_hashes: Vec<sha256d::Hash>,
    /// Stop_hash from sende viewpoint
    pub stop_hash:      sha256d::Hash
}


impl SendHeadersMessage {
    pub fn new_send_headers_msg(block_location_hashes: Vec<sha256d::Hash>, stop_hash: sha256d::Hash) -> SendHeadersMessage {
        if block_location_hashes.len() > MAX_BLOCK_HEADERS_PER_MSG {
            //
        }
        //
    }
}

/// `getblocks` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GetBlocksMessage {
    /// Protocol version
    pub protocol: u32,

    /// Block hashes ordered from newest to oldest
    /// this data is sent according to the longest chain the peer sees
    pub block_locator_hashes: Vec<sha256d::Hash>,

    /// The block hash to stop at; if 0 - fetch the maximum of 500 blocks
    pub hash_stop: sha256d::Hash,
}

impl GetBlocksMessage {
    pub fn new_get_blocks_msg(block_location_hashes: Vec<sha256d::Hash>, hash_stop: sha256d::Hash) -> GetBlocksMessage {
        GetBlocksMessage {
            protocol:             constants::PROTOCOL_VERSION,
            block_locator_hashes: block_location_hashes,
            hash_stop:            hash_stop
        }
    }
}

consensus_encoding!(GetBlocksMessage, protocol, block_location_hashes, hash_stop);