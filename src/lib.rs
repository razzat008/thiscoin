use serde::Deserialize;
use uint::construct_uint;
construct_uint! {
   // Construct an unsigned 256-bit integer
   // consisting of 4 x 64-bit words
    #[derive(serde::Serialize,Deserialize)]
   pub struct U256(4);
}

// reward awarded to the miner
pub const INITIAL_REWARD: u64 = 50;
// interval when the reward is halved
pub const HALVING_INTERVAL: u64 = 210;
// time it takes for the network to generate a new block and add that block to the blockchain;
// average time taken to mine a new block 
pub const IDEAL_BLOCK_TIME: u64 = 10; // this is in seconds unlike the actual bitcoin protocol
                                      // which is 10 minutes

// easiest target for a miner
// little-endian notation; latter position -> earlier digits
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF, //smallest value
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF, // largest value
]);

// difficulty update interval for each block
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

extern crate ciborium;
extern crate serde;
pub mod crypto;
pub mod error;
pub mod sha256;
pub mod types;
pub mod utils;

