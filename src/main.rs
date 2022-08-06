use ed25519_dalek::Keypair;
use simple_blockchain_rs::client::Client;
use simple_blockchain_rs::transaction::Transaction;

fn main() {
    let client = Client::new();
    //println!("client public key: {:?}", client.identify());

    let receiver = Client::new();
    let amount = 1.0;

    let mut transaction1 = Transaction::new(client.identify(), receiver.identify(), amount, None);
    transaction1.sign_transaction(Keypair {
        secret: client.secret,
        public: client.public_key,
    });

    //transaction1.print_transaction();
    //transaction1.print_signature();
}
