use super::transaction::*;
use serde::{Deserialize, Serialize};

const DIFFICULTY_STRING: &str = "0";
pub const GENESIS_BLOCK_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// A block in the blockchain.
///
/// `index` contains the index of the block.
/// `nonce` contains the nonce of the block that is used to find a valid hash.
/// `previous_block_hash` contains the hash of the previous block.
/// `hash` contains the hash of the block.
/// `verified_transactions` contains the transactions that are verified in the block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub nonce: usize,
    pub previous_block_hash: String,
    pub block_hash: String,
    pub verified_transactions: Vec<Transaction>,
}

impl Block {
    /// This method creates a new block.
    pub fn new(index: usize, previous_block_hash: String) -> Self {
        Self {
            index,
            nonce: 0,
            previous_block_hash,
            block_hash: String::new(),
            verified_transactions: vec![],
        }
    }

    pub fn get_block_reward(index: usize) -> f64 {
        let block_factor = 100; /* bitcoin has block factor of 210000 */
        50.0 / (2.0_f64.powi(index as i32 / block_factor) as f64)
    }

    pub fn verify_block(&self) -> Result<(), &'static str> {
        if !self.block_hash.starts_with(DIFFICULTY_STRING) {
            return Err("Block verification failed: PoW is not valid");
        }

        if self.calculate_hash() != self.block_hash {
            return Err("Block verification failed: Block hash is not valid");
        }

        Ok(())
    }

    /// This method verifies the coinbase transaction of genesis block.
    pub fn verify_coinbase_transaction(&self) -> Result<(), &'static str> {
        if self.verified_transactions.len() == 0 {
            return Err("Block verification failed: No coinbase transaction");
        }
        if self.verified_transactions.iter().next().unwrap().amount != Self::get_block_reward(0) {
            return Err("Block verification failed: Coinbase transaction amount is not valid");
        }

        Ok(())
    }

    /// This method verifies the transactions inside the block.
    pub fn has_valid_transactions(&self) -> Result<(), &'static str> {
        if self.index == 0 {
            return self.verify_coinbase_transaction();
        }

        for transaction in self.verified_transactions.iter() {
            transaction.is_valid_transaction()?;
        }
        Ok(())
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

    pub fn mine_block(self, difficulty_level: usize) -> Result<Block, &'static str> {
        let mut nonce = 0;
        let mut block = self.clone();

        loop {
            let hash = block.calculate_hash();

            if hash.starts_with(&DIFFICULTY_STRING.repeat(difficulty_level)) {
                block.nonce = nonce;
                block.block_hash = hash;
                break;
            }

            if nonce > 10000 {
                return Err("Difficulty is too high! block mining failed.");
            }
            nonce = nonce + 1;
            block.nonce = nonce;
        }
        Ok(block)
    }
}
