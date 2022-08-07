use super::transaction::*;
use crate::client::*;

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

    pub fn genesis_block(receiver: &Client) -> Self {
        let genesis_hash =
            "0000000000000000000000000000000000000000000000000000000000000001".to_string();
        let genesis_secp = Secp256k1::new();
        let genesis_secretkey = key::SecretKey::from_str(genesis_hash.as_str()).unwrap();
        let genesis_publickey = key::PublicKey::from_secret_key(&genesis_secp, &genesis_secretkey);

        let initial_transaction =
            Transaction::new(genesis_publickey, receiver.public_key, 1000.0, None);

        let mut genesis_block = Block::new(0, genesis_publickey.to_string());

        genesis_block
            .verified_transactions
            .push(initial_transaction);

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
            self.index, self.nonce, self.pre_hash, transactions
        )
    }

    /// This method calculates the hash of the block using SHA256.
    pub fn calculate_hash(&self) -> String {
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &self.serialize().as_bytes())
    }
}
