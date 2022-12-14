use std::str::FromStr;

pub use secp256k1::ecdsa::Signature;
pub use secp256k1::{rand, All, Secp256k1};
pub use secp256k1::{PublicKey, SecretKey};

/// A client structure that can be used to interact with a blockchain.
///
/// `public_key` contains the public key of the client.
/// `secret_key` contains the private key of the client.
/// `secp` contains the secp256k1 context.
pub struct Client {
    pub secp: Secp256k1<All>,
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl Client {
    /// This method creates a new client with a random key pair.
    pub fn new() -> Self {
        let mut rng = rand::rngs::OsRng {};
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Self {
            secp,
            secret_key,
            public_key,
        }
    }

    pub fn from(key: String) -> Result<Self, secp256k1::Error> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(&key)?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Self {
            secp,
            secret_key,
            public_key,
        })
    }

    /// This method identifies the client to the blockchain.
    pub fn identify(&self) -> String {
        self.public_key.to_string()
    }

    /// This method signs a transaction with the client's private key.
    pub fn sign(&self, transaction: &[u8]) -> Signature {
        let message = secp256k1::Message::from_slice(transaction).unwrap();
        self.secp.sign_ecdsa(&message, &self.secret_key)
    }
}
