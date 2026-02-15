construct_uint! {
   // Construct an unsigned 256-bit integer
   // consisting of 4 x 64-bit words
   pub struct U256(4);
}
use uint::construct_uint;
extern crate ciborium;
extern crate serde;
extern crate sha256;
pub mod types;
