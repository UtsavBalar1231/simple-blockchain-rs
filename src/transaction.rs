use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use rand::AsByteSliceMut;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: PublicKey,
    pub receiver: PublicKey,
    time: DateTime<Utc>,
    pub amount: f64,
    signature: Option<Signature>,
}

impl Transaction {
    pub fn new(
        sender: &[u8; 32],
        receiver: &[u8; 32],
        amount: f64,
        signature: Option<Signature>,
    ) -> Self {
        
        if sender == receiver {
            panic!("Sender and receiver cannot be the same.");
        }

        Self {
            sender: PublicKey::from_bytes(sender).unwrap(),
            receiver: PublicKey::from_bytes(receiver).unwrap(),
            time: Utc::now(),
            signature,
            amount,
        }
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.sender.as_bytes());
        data.extend(self.receiver.as_bytes());
        data.extend(self.time.to_rfc3339().as_bytes());
        if let Some(signature) = &self.signature {
            data.extend(signature.to_bytes());
        }
        data.extend(&self.amount.to_bits().to_ne_bytes());
        println!("data: {:?}", String::from_utf8_lossy(data.as_byte_slice_mut()));
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_slice())
    }

    pub fn sign_transaction(&mut self, key: Keypair) {
        if self.sender != key.public {
            panic!("You can not sign other's transaction!!!")
        } else {
            self.signature = Some(key.sign(&self.calculate_hash()));
        }
    }
}
