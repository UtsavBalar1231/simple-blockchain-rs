use super::transaction::*;
use crate::client::*;

/// A block in the blockchain.
///
/// `index` contains the index of the block.
/// `nonce` contains the nonce of the block that is used to find a valid hash.
/// `previous_hash` contains the hash of the previous block.
/// `hash` contains the hash of the block.
/// `verified_transactions` contains the transactions that are verified in the block.
#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub previous_hash: Option<String>,
    pub hash: String,
    pub verified_transactions: Vec<Transaction>,
}

impl Block {
    /// This method creates a new block.
    pub fn new(index: u64, previous_hash: Option<String>) -> Self {
        Self {
            index,
            nonce: 0u64,
            previous_hash,
            hash: String::new(),
            verified_transactions: vec![],
        }
    }

    /// This method generates genesis block.
    pub fn genesis_block(receiver: &Client) -> Self {
        let genesis = Client::new();

        let initial_transaction =
            Transaction::new(genesis.public_key, receiver.public_key, 1000.0, None);

        let mut genesis_block = Block::new(0, None);

        genesis_block
            .verified_transactions
            .push(initial_transaction);

        genesis_block.hash = genesis_block.calculate_hash();
        genesis_block
    }

    /// This method verifies the transactions in the block.
    pub fn has_valid_transactions(&self) -> bool {
        for tran in &self.verified_transactions {
            if !tran.is_valid_transaction() {
                return false;
            }
        }

        return true;
    }

    /// This method serializes the block into a string.
    pub fn serialize(&self) -> String {
        let transactions = self
            .verified_transactions
            .iter()
            .fold(String::new(), |acc, x| acc + &x.serialize());
        format!(
            "{}{}{}{}",
            self.index,
            self.nonce,
            self.previous_hash.as_ref().unwrap_or(&String::new()),
            transactions
        )
    }

    /// This method calculates the hash of the block using SHA256.
    pub fn calculate_hash(&self) -> String {
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &self.serialize().as_bytes())
    }
}
