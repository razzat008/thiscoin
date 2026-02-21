#![allow(unused)]
use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey, signature::rand_core::OsRng};
use k256::Secp256k1;
use rand::{self, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug)]
pub struct PrivateKey(SigningKey<Secp256k1>);

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

impl PrivateKey {
    pub fn new() -> Self {
        PrivateKey(SigningKey::random(&mut OsRng))
    }

    pub fn public_key(&self) -> PublicKey {
        // get the correspoing public key
        PublicKey(*self.0.verifying_key())
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}
