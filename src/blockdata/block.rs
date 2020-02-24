/// Rust Blockchain Sample Project

use hashes::{sha256d, Hash};

use util;
use blockdata::Error::{BlockProofOfWorkError, BlockTargetError};
use util::hash::{BlockchainHash, MerkleRoot, blockchain_merkle_root};
use util::uint::Uint256; // not implemented

use blockdata::transaction::Transaction;

/// 'BlockHeader' representation
/// the BlockHeader contains all information related to the 
/// block except actual transactions
#[derive(Copy, PartialEq, Debug, Clone, Eq, Hash)]
pub struct BlockHeader {
    ///  Version of the block; not the same as protocol version
    pub block_version: u32,
    /// Hash of previous block on the best chain
    pub prev_blockhash: sha256d::Hash,
    /// Merkle tree root hash
    pub merkle_root: sha256d::Hash,
    /// Time of creation of the current block - as per miner claim
    pub timestamp: u32,
    /// Height of the current block
    pub height: u64,
    /// Target below which the block hash must lie
    pub target: u32,
    /// Nonce used to calculate the blockchash
    pub nonce: u32
}

/// A 'Block' implementation
/// contains transactions and block header
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Transactions contained in the block
    pub txdata: Vec<Transaction>
}

impl Block {
    /// Check if header merkle root matched the merkle root of the transactions
    pub fn check_merkle_root (&self) -> bool {
        self.header.merkle_root == self.merke_root()
    }

    /// Check if witness commitment in coinbase is matching the transactions list
    pub check_witness_commit(&self) -> bool {
        /// ...
    }
    false
}

imple MerkleRoot for Block {
    fn merkle_root(&self) -> sha256d::Hash {
        blockchain_merkle_root(self.txdata.iter().map(|obj| obj.txid()).collect())
    }
}

impl BlockHeader {
    /// Computes the target [0, T] that a blockhash must land in to be valid
    pub fn compute_target(&self) -> u256 {
        // This is a floating-point "compact" encoding originally used by
        // OpenSSL, which satoshi put into consensus code
        // The exponent needs to have 3 subtracted from it, hence
        // this goofy decoding code:
        let (mant, expt) = {
            let unshifted_expt = self.bits >> 24;
            if unshifted_expt <= 3 {
                ((self.bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (self.bits & 0xFFFFFF, 8 * ((self.bits >> 24) - 3))
            }
        };

        // The mantissa is signed but may not be negative
        if mant > 0x7FFFFF {
            Default::default()
        } else {
            Uint256::from_u64(mant as u64).unwrap() << (expt as usize)
        }
    }

    /// Computes the target value in float format from Uint256 format.
    pub fn compute_compact_target_from_u256(value: &Uint256) -> u32 {
        let mut size = (value.bits() + 7) / 8;
        let mut compact = if size <= 3 {
            (value.low_u64() << (8 * (3 - size))) as u32
        } else {
            let bn = *value >> (8 * (size - 3));
            bn.low_u32()
        };

        if (compact & 0x00800000) != 0 {
            compact >>= 8;
            size += 1;
        }

        compact | (size << 24) as u32
    }

    /// Compute the popular "difficulty" measure for mining
    pub fn compute_difficulty(&self, network: Network) -> u64 {
        // implement max_target()
        (max_target(network) / self.target()).low_u64()
    }

    /// Validate the block proof of work
    pub fn validate_pow(&self, target_required: &Uint256) -> Result<(), util::Error> {
        use byteorder::{ByteOrder, LittleEndian};

        let target =&self.compute_target();
        if target != target_required {
            return Err(BlockTargetError);
        }
        let data: [u8; 32] = self.blockchain_hash().into_inner();
        let mut ret = [0u64; 4];
        LittleEndian::read_u64_into(&data, &mut ret);
        let hash = &Uint256(ret);
        if hash <= target { Ok(()) } else { Err(BlockProofOfWorkError) }
    }

    /// Return the total work of the block
    pub fn total_work(&self) -> Uint256 {
        // 2**256 / (target + 1) == ~target / (target+1) + 1    (eqn shamelessly stolen from bitcoind)
        let mut ret = !self.target();
        let mut ret1 = self.target();
        ret1.increment();
        ret = ret / ret1;
        ret.increment();
        ret
    }

    /// Construct the next block
    pub fn next(&self, value: T) -> Block {
        // implement
    }
}

impl BlockchainHash for BlockHeader {
    fn blockchain_hash(&self) -> sha256d::Hash {
        // implement serialize()
        sha256d::Hash::hash(&serialize(self))
    }
}

impl BlockchainHash for Block {
    fn blockchain_hash(&self) -> sha256d::Hash {
        self.header.blockchain_hash()
    }
}


