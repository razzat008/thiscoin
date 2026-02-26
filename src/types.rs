#![allow(dead_code, unused)]

use std::collections::HashMap;

use crate::crypto::{PublicKey, Signature};
use crate::error::{Result, ThisCoinError};
use crate::sha256::Hash;
use crate::{U256, utils::MerkleRoot};
use chrono::{DateTime, Utc};
use ecdsa::SigningKey;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//each bloack have blockheader
#[derive(Serialize, Clone, Deserialize, Debug)]
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

// transactions
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct TransactionInput {
    pub prev_trans_hash: Hash, // hash of the previous transaction | creating a chain
    pub signature: Signature,  // SHA256, the signature of the user
}

// some value(the transaction)
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    // each transaction will have some input and output
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

// bolockchain
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>, //unspent transaction output
    pub blocks: Vec<Block>,                      // blocks in the Blockchain
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            utxos: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        // checking a block's validity
        if self.blocks.is_empty() && block.header.prev_block_hash != Hash::zero() {
            return Err(ThisCoinError::InvalidBlock);
        } else if let Some(last_block) = self.blocks.last() {
            if last_block.hash() != block.header.prev_block_hash {
                eprint!("previous block's hash is wrong");
                return Err(ThisCoinError::InvalidBlock);
            }

            if !block.header.hash().matches_target(block.header.target) {
                print!("doesn't match target");
                return Err(ThisCoinError::InvalidBlock);
            }
            let calc_merkle_root = MerkleRoot::calculate_merkleroot(&block.transactions);
            if calc_merkle_root != block.header.merkle_root {
                return Err(ThisCoinError::InvalidMerkelRoot);
            }

            if block.header.timestamp <= last_block.header.timestamp {
                return Err(ThisCoinError::InvalidBlock);
            }

            block.verify_transactions(self.blocks, &self.utxos);
        }
        self.blocks.push(block);
        Ok(())
    }

    pub fn rebuild_uxtos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    // destorying the coin (creating chunks)
                    self.utxos.remove(&input.prev_trans_hash);
                }
                // giving remaining coin back
                for output in transaction.outputs.iter() {
                    self.utxos.insert(transaction.hash(), output.clone());
                }
            }
        }
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

// each block in a blockchain
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }

    pub fn verify_transactions(
        &self,
        block_height: u8,
        utxos: &HashMap<Hash, TransactionOutput>,
    ) -> Result<()> {
        todo!();
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
