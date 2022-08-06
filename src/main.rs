use ed25519_dalek::Keypair;
use simple_blockchain_rs::block::Block;
use simple_blockchain_rs::blockchain::Blockchain;
use simple_blockchain_rs::client::Client;
use simple_blockchain_rs::transaction::Transaction;

fn main() {
    // let client = Client::new();
    //println!("client public key: {:?}", client.public_key);

    // let receiver = Client::new();
    // let amount = 1.0;

    // let mut transaction1 = Transaction::new(client.public_key, receiver.public_key, amount, None);
    // transaction1.sign_transaction(Keypair {
    //     secret: client.secret,
    //     public: client.public_key,
    // });

    //transaction1.print_transaction();
    //transaction1.print_signature();

    //let mut transactions = vec![];

    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let genesis = Keypair::from_bytes(
        "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".as_bytes(),
    )
    .unwrap()
    .public;

    let transaction1 = Transaction::new(genesis, utsav.public_key, 1000.0, None);

    let mut block0 = Block::new(0);
    block0.pre_hash = None;
    block0.nonce = None;
    block0.verified_transactions.push(transaction1);
    block0.sign_block();

    println!("utsav public key: {:?}", utsav.public_key.as_bytes());
    println!(
        "bhupendra public key: {:?}",
        bhupendra.public_key.as_bytes()
    );
    println!("jash public key: {:?}", jash.public_key.as_bytes());
    println!("");

    let mut transaction2 = Transaction::new(utsav.public_key, bhupendra.public_key, 10.0, None);
    transaction2.sign_transaction(Keypair {
        secret: utsav.secret,
        public: utsav.public_key,
    });
    block0.verified_transactions.push(transaction2);

    let mut transaction3 = Transaction::new(bhupendra.public_key, jash.public_key, 10.0, None);
    transaction3.sign_transaction(Keypair {
        secret: bhupendra.secret,
        public: bhupendra.public_key,
    });
    block0.verified_transactions.push(transaction3);

    let mut transaction4 = Transaction::new(jash.public_key, utsav.public_key, 10.0, None);
    transaction4.sign_transaction(Keypair {
        secret: jash.secret,
        public: jash.public_key,
    });
    block0.verified_transactions.push(transaction4);

    let _last_block_hash = block0.signature.clone().unwrap();
    let mut coin_chain = Blockchain::new();
    coin_chain.add_block(block0);
    coin_chain.dump_blockchain();
}
