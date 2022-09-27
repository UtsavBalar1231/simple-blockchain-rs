use crate::client::*;
use chrono::{DateTime, Utc};
use secp256k1::Message;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};

/// A transaction structure that can be used to record a transaction in the blockchain.
///
/// `sender` contains the public key of the client that is sending the transaction.
/// `receiver` contains the public key of the client that is receiving the transaction.
/// `amount` contains the amount of money that is being sent.
/// `signature` contains the signature of the transaction.
/// `timestamp` contains the time at which the transaction was created.
#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: Option<PublicKey>,
    pub receiver: PublicKey,
    time: DateTime<Utc>,
    pub amount: f64,
    signature: Option<String>,
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("sender", &format!("{}", self.sender.unwrap()))
            .field("receiver", &format!("{}", self.receiver))
            .field("time", &self.time)
            .field("amount", &self.amount)
            .field("signature", &self.signature.as_ref().unwrap())
            .finish()
    }
}

impl Transaction {
    /// This method creates a new transaction.
    pub fn new(
        sender: Option<PublicKey>,
        receiver: PublicKey,
        amount: f64,
        signature: Option<String>,
    ) -> Self {
        Self {
            sender,
            receiver,
            time: Utc::now(),
            signature,
            amount,
        }
    }

    /// This method creates a new transaction.
    pub fn signed_new(sender: &Client, receiver: PublicKey, amount: f64) -> Self {
        let mut transaction = Transaction::new(Some(sender.public_key), receiver, amount, None);
        transaction.sign_transaction(&sender);

        transaction
    }

    /// This method serializes the transaction into a string.
    pub fn serialize_transaction(&self) -> String {
        let sender = match &self.sender {
            Some(sender) => sender.to_string(),
            None => String::new(),
        };

        format!("{}{}{}{}", sender, self.receiver, self.amount, self.time,)
    }

    /// This method calculates the hash of the transaction using SHA256.
    pub fn calculate_hash(&self) -> Vec<u8> {
        crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &self.serialize_transaction().as_bytes(),
        )
    }

    /// This method signs the transaction using the private key of the signer.
    pub fn sign_transaction(&mut self, signer: &Client) {
        self.signature = signer.sign(&self.calculate_hash()).to_string().into();
    }

    /// This method prints the signature of the transaction.
    pub fn print_transaction(&self) {
        if let Some(sender) = &self.sender {
            println!("sender: {}", sender.to_string());
        }
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
    pub fn is_valid_transaction(&self) -> Result<(), &'static str> {
        if self.signature.is_none() {
            return Err("No signature found.");
        }

        let secp = Secp256k1::verification_only();

        let unsigned_transaction_hash =
            Message::from_slice(self.calculate_hash().as_slice()).unwrap();

        let sig =
            Signature::from_str(self.signature.as_ref().expect("No signature found.")).unwrap();

        match secp.verify_ecdsa(&unsigned_transaction_hash, &sig, &self.sender.unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => {
                println!("Transaction is invalid: {:?}", self);
                Err("Invalid signature.")
            }
        }
    }
}
