use crate::client::Client;
use chrono::{DateTime, Utc};
use secp256k1::{Message, PublicKey, Secp256k1};
use secp256k1::ecdsa::Signature;
use std::str::FromStr;

/// A transaction structure that can be used to record a transaction in the blockchain.
///
/// `sender` contains the public key of the client that is sending the transaction.
/// `receiver` contains the public key of the client that is receiving the transaction.
/// `amount` contains the amount of money that is being sent.
/// `signature` contains the signature of the transaction.
/// `timestamp` contains the time at which the transaction was created.
#[derive(Debug)]
pub struct Transaction {
    pub sender: PublicKey,
    pub receiver: PublicKey,
    time: DateTime<Utc>,
    pub amount: f64,
    signature: Option<String>,
}

impl Transaction {
    /// This method creates a new transaction.
    pub fn new(
        sender: PublicKey,
        receiver: PublicKey,
        amount: f64,
        signature: Option<String>,
    ) -> Self {
        if sender == receiver {
            panic!("Sender and receiver cannot be the same.");
        }

        Self {
            sender,
            receiver,
            time: Utc::now(),
            signature,
            amount,
        }
    }

    /// This method serializes the transaction into a string.
    pub fn serialize(&self) -> String {
        format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.time,
        )
    }

    /// This method calculates the hash of the transaction using SHA256.
    pub fn calculate_hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.serialize().as_bytes())
    }

    /// This method signs the transaction using the private key of the signer.
    pub fn sign_transaction(&mut self, signer: &Client) {
        self.signature = Some(signer.sign(&self.calculate_hash()).to_string());
    }

    /// This method prints the signature of the transaction.
    pub fn print_transaction(&self) {
        println!("sender: {}", self.sender.to_string());
        println!("receiver: {}", self.receiver.to_string());
        println!("time: {:?}", self.time);
        println!("amount: {:?}", self.amount);
        if let Some(signature) = &self.signature {
            println!("signature: {}", signature);
        }
        println!("");
    }

    /// This method prints the signature of the transaction.
    pub fn print_signature(&self) {
        println!(
            "Signature: {}",
            self.signature.as_ref().expect("No signature found.")
        );
    }

    /// This method verifies the signature of the transaction.
    pub fn is_valid_transaction(&self) -> bool {
        let secp = Secp256k1::verification_only();
        let unsigned_tx_hash =
            Message::from_slice(self.calculate_hash().as_slice()).expect("message from slice");
        if let Some(_) = &self.signature {
            let sig =
                Signature::from_str(self.signature.as_ref().unwrap_or(&String::new()).as_str())
                    .expect("signature from string");
            return secp.verify_ecdsa(&unsigned_tx_hash, &sig, &self.sender).is_ok();
        } else {
            return false;
        }
    }
}
