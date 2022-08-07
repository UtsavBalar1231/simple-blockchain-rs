use super::transaction::Transaction;

/// A block in the blockchain.
/// 
/// `index` contains the index of the block.
/// `previous_hash` contains the hash of the previous block.
/// `transactions` contains the transactions in the block.
/// `hash` contains the hash of the block.
/// `nonce` contains the nonce of the block.
#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub pre_hash: String,
    pub hash: String,
    pub verified_transactions: Vec<Transaction>,
}

impl Block {
    /// This method creates a new block.
    pub fn new(index: u64, pre_hash: String) -> Self {
        Self {
            index,
            nonce: 0u64,
            pre_hash,
            hash: String::new(),
            verified_transactions: vec![],
        }
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
            self.index, self.nonce, self.pre_hash, transactions
        )
    }

    /// This method calculates the hash of the block using SHA256.
    pub fn calculate_hash(&self) -> String {
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &self.serialize().as_bytes())
    }
}
