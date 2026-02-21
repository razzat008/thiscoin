use crate::{sha256::Hash, types::Transaction};
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate_merkleroot(transactions: &[Transaction]) -> MerkleRoot {
        let mut layer: Vec<Hash> = vec![];
        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }
        while layer.len() > 1 {
            let mut preceeding_layer: Vec<Hash> = vec![];
            for pair in layer.chunks(2) {
                let left_node = pair[0]; // hash of left transaction
                let right_node = *pair.get(1).unwrap_or(&pair[0]); // hash for two transactions
                // BB(if only one transaction is
                // seen)
                preceeding_layer.push(Hash::hash(&[left_node, right_node]));
            }
            layer = preceeding_layer
        }
        // return the first hash | if only one layer root is that itself
        MerkleRoot(layer[0])
    }
}
