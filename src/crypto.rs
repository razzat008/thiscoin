#![allow(unused)]

use ecdsa::{
    Signature as ECDSASignature, SigningKey, VerifyingKey,
    signature::{Signer, Verifier, rand_core::OsRng},
};
use k256::Secp256k1;
use rand::{self, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::sha256::Hash;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug)]
pub struct PrivateKey(SigningKey<Secp256k1>);

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl Signature {
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        // next owner's signature ( previous transaction's hash + public key of next owner)
        let sigin_key = &private_key.0;
        let signature = sigin_key.sign(&output_hash.clone().as_bytes());
        Signature(signature)
    }

    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        // verifying the hash with previous owner's public key
        public_key
            .0
            .verify(&output_hash.clone().as_bytes(), &self.0)
            .is_ok()
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}
