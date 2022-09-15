use super::transaction::*;
use crate::client::*;

/// A block in the blockchain.
///
/// `index` contains the index of the block.
/// `nonce` contains the nonce of the block that is used to find a valid hash.
/// `previous_block_hash` contains the hash of the previous block.
/// `hash` contains the hash of the block.
/// `verified_transactions` contains the transactions that are verified in the block.
#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub previous_block_hash: String,
    pub hash: String,
    pub verified_transactions: Vec<Transaction>,
}

impl Block {
    /// This method creates a new block.
    pub fn new(index: u64, previous_block_hash: String) -> Self {
        Self {
            index,
            nonce: 0u64,
            previous_block_hash,
            hash: String::new(),
            verified_transactions: vec![],
        }
    }

    /// This method generates genesis block.
    pub fn genesis_block(receiver: &Client) -> Self {
        let genesis = Client::new();

        let initial_transaction =
            Transaction::new(genesis.public_key, receiver.public_key, 1000.0, None);

        let mut genesis_block = Block::new(0, String::from("0").repeat(64));

        genesis_block
            .verified_transactions
            .push(initial_transaction);

        genesis_block.hash = genesis_block.previous_block_hash.clone();
        genesis_block
    }

    /// This method verifies the transactions in the block.
    pub fn has_valid_transactions(&self) -> bool {
        for transaction in &self.verified_transactions {
            if !transaction.is_valid_transaction() {
                return false;
            }
        }

        return true;
    }

    /// This method serializes the block into a string.
    pub fn serialize_block(&self) -> String {
        let transactions = self
            .verified_transactions
            .iter()
            .fold(String::new(), |acc, t| acc + &t.serialize_transaction());

        format!(
            "{}{}{}{}",
            self.index, self.nonce, self.previous_block_hash, transactions
        )
    }

    /// This method calculates the hash of the block using SHA256.
    pub fn calculate_hash(&self) -> String {
        crypto_hash::hex_digest(
            crypto_hash::Algorithm::SHA256,
            &self.serialize_block().as_bytes(),
        )
    }
}
