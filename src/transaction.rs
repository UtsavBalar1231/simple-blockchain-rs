use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};

/// A transaction structure that can be used to record a transaction in the blockchain.
///
/// `sender` contains the public key of the client that is sending the transaction.
/// `receiver` contains the public key of the client that is receiving the transaction.
/// `amount` contains the amount of money that is being sent.
/// `signature` contains the signature of the transaction.
/// `timestamp` contains the time at which the transaction was created.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: PublicKey,
    pub receiver: PublicKey,
    time: DateTime<Utc>,
    pub amount: f64,
    signature: Option<Signature>,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction {{ sender: {:?}, receiver: {:?}, amount: {:?}, signature: {:?}, time: {:?} }}",
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            self.amount,
            self.signature.as_ref().map(|s| s.to_bytes()),
            self.time
        )
    }
}

impl Transaction {
    pub fn new(
        sender: PublicKey,
        receiver: PublicKey,
        amount: f64,
        signature: Option<Signature>,
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

    pub fn bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.sender.as_bytes());
        data.extend(self.receiver.as_bytes());
        data.extend(self.time.to_rfc3339().as_bytes());
        if let Some(signature) = &self.signature {
            data.extend(signature.to_bytes());
        }
        data.extend(&self.amount.to_bits().to_ne_bytes());
        data
    }

    /// This method calculates the hash of the transaction using SHA256.
    pub fn calculate_hash(&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }

    /// This method signs the transaction using the private key of the client.
    pub fn sign_transaction(&mut self, key: Keypair) {
        if self.sender != key.public {
            panic!("You can not sign other's transaction!!!")
        } else {
            self.signature = Some(key.sign(&self.calculate_hash()));
        }
    }

    /// This method prints the signature of the transaction.
    pub fn print_transaction(&self) {
        println!("sender: {:?}", self.sender.as_bytes());
        println!("receiver: {:?}", self.receiver.as_bytes());
        println!("time: {:?}", self.time);
        println!("amount: {:?}", self.amount);
        println!("signature: {:#?}", self.signature);
    }

    /// This method prints the signature of the transaction.
    pub fn print_signature(&self) {
        println!("{:?}", self.signature.expect("No signature found."));
    }

    pub fn is_valid_transaction(&self) -> bool {
        match (self.sender, self.signature) {
            (p, Some(s)) if p.verify(&self.calculate_hash(), &s).is_ok() => true,
            _ => false,
        }
    }
}
