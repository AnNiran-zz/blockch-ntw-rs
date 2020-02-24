/// Hash functions

use std::default::Default;

use hashes::{sha256d, Hash};

/// Any collection of objects for which a merkle root makes sense to be calculated
pub trait MerkleRoot {
    /// Construct a merkle tree from a collection of elements, keeping their initial order
    /// Return the merkle root
    fn merkle_root(&self) -> sha256d::Hash;
}

/// Return merke root for transactions ids list
pub fn blockchain_merkle_root(data: Vec<sha256d::Hash>) -> sha256d::Hash {
    if data.len() < 1 {
        return Default::default();
    }
    if data.len() < 2 {
        return data[0];
    }
    let mut next_pos = vec![];
    for index in 0..((data.len() + 1) / 2) {
        let index1 = 2 * index;
        let index2 = min(index1 + 1, data.len() - 1);
        let mut encoder = sha256d::Hash::engine();
        data[index1].encode(&mut encoder).unwrap();
        data[index2].encode(&mut encoder).unwrap();
        next_pos.push(sha256d::Hash::from_engine(encoder));
    }
    blockchain_merkle_root(next_pos)
}

/// Used for objects referable by their hashes
pub trait BlockchainHash {
    /// Return 256-bit hash from the object
    fn blockchain_hash(&self) -> sha256d::Hash;
}