#![allow(dead_code, unused)]

use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::{U256, utils::MerkleRoot};
use chrono::{DateTime, Utc};
use ecdsa::SigningKey;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>, // timestamp of the block | each timestamp includes the previous timestamp in
    // it's hash, hence forming a chain
    pub nonce: u64, // number incremented to mine the block, when hash of the current has is less than
    // the target
    pub prev_block_hash: Hash,   // hash of the previous block
    pub merkle_root: MerkleRoot, // the source of truth | hash tree propagating upwards
    pub target: U256, // "threshold" of how much smaller a block's hash needs to be | adjusted by
                      // Difficulty Adjustment
}

impl BlockHeader {
    fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionInput {
    pub prev_trans_hash: Hash, // hash of the previous transaction | creating a chain
    pub signature: Signature,  // SHA256, the signature of the user
}

// some value(the transaction)
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub uniq_id: Uuid, // identifier to ensure hash of each transaction is unique
    pub pubkey: PublicKey, // to sign/verify
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    // each transaction will have some input and output
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transaction: Transaction,
}

impl Block {
    pub fn new(header: BlockHeader, transaction: Transaction) -> Self {
        Self {
            header,
            transaction,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>);

// implementing (de)serializers for our Key
mod signkey_serde {
    use serde::Deserialize;
    use serde_json::Deserializer;

    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 32] = Deserialize::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
