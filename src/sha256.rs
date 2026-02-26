use std::fmt::Display;

use crate::U256;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Hash(U256);

impl Hash {
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!(
                "Failed to serialize data: {:?}.\
                This is awkward...",
                e
            );
        }
        let hash = Sha256::digest(serialized);
        let val = U256::from_big_endian(&hash[..]);
        Hash(val)
    }

    // checking if the computed Hash is less than the target | hence solving it
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    pub fn as_bytes(&mut self) -> [u8; 32] {
         self.0.to_little_endian()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
