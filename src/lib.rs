construct_uint! {
   // Construct an unsigned 256-bit integer
   // consisting of 4 x 64-bit words
   pub struct U256(4);
}
use uint::construct_uint;
extern crate ciborium;
extern crate serde;
pub mod types;
pub mod crypto;
pub mod utils;
pub mod sha256;
