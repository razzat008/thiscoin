use crate::U256;

#[allow(dead_code)]
pub struct BlockHeader {
    timestamp: u64, // timestamp of the block
    nonce: u64, //number incremented to mine the block
    prev_block_hash: [u8; 32], // hash of the previous block
    merkle_root: [u8; 32], // idk
    pub target: U256, //
}

pub struct Transaction;

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
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
