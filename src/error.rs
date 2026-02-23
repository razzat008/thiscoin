use thiserror::Error;

#[derive(Debug, Error)]
#[allow(non_snake_case)]
pub enum ThisCoinError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Invalid block header")]
    InvalidBlockHeader,
    #[error("Invalid transaction input")]
    InvalidTransactionInput,
    #[error("Invalid transaction output")]
    InvalidTransactionOutput,
    #[error("Invalid Merkle root")]
    InvalidMerkelRoot,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid public key")]
    InvalidPublickKey,
    #[error("Invalid private key")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, ThisCoinError>;
