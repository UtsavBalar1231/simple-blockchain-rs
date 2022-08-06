use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, SecretKey};
use rand::rngs::OsRng;

/// A client structure that can be used to interact with a blockchain.
///
/// `key_pair` contains the private key and public key of the client.
/// The public key is used to identify the client to the blockchain.
/// The private key is used to sign messages.

pub struct Client {
    pub secret: SecretKey,
    pub public_key: PublicKey,
    pub signer: Signature,
}

impl Client {
    /// Creates a new client with a random key pair.
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let key_pair = Keypair::generate(&mut csprng);
        let signer = key_pair.sign(b"");
        let public_key = key_pair.public;
        Self {
            secret: key_pair.secret,
            public_key,
            signer,
        }
    }

    /// Identifies the client to the blockchain.
    pub fn identify(&self) -> &[u8; 32] {
        self.public_key.as_bytes()
    }
}
