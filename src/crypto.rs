#![allow(unused)]
use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey, signature::Signer};
use k256::Secp256k1;

pub struct Signature(ECDSASignature<Secp256k1>);
pub struct PrivateKey(ECDSASignature<Secp256k1>);
pub struct PublicKey(ECDSASignature<Secp256k1>);
