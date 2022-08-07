use std::str::FromStr;

use secp256k1::{PublicKey, Secp256k1, SecretKey};
use simple_blockchain_rs::block::Block;
use simple_blockchain_rs::blockchain::Blockchain;
use simple_blockchain_rs::client::Client;
use simple_blockchain_rs::transaction::Transaction;

fn main() {
    // let client = Client::new();
    // println!("client public key: {}", client.identify());

    // let receiver = Client::new();
    // let amount = 1.0;

    // let mut transaction1 = Transaction::new(client.public_key, receiver.public_key, amount, None);
    // transaction1.sign_transaction(&client);

    // println!("{:#?}", transaction1);
    //transaction1.print_signature();

    //let mut transactions = vec![];

    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let genesis_hash =
        "0000000000000000000000000000000000000000000000000000000000000001".to_string();
    let genesis_secp = Secp256k1::new();
    let genesis_secretkey = SecretKey::from_str(genesis_hash.as_str()).unwrap();
    let genesis_publickey = PublicKey::from_secret_key(&genesis_secp, &genesis_secretkey);
    let transaction0 = Transaction::new(genesis_publickey, utsav.public_key, 1000.0, None);

    let mut block0 = Block::new(0, genesis_hash);
    block0.verified_transactions.push(transaction0);

    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());
    println!("");

    let mut transaction1 = Transaction::new(utsav.public_key, bhupendra.public_key, 10.0, None);
    transaction1.sign_transaction(&utsav);
    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction1);

    let mut transaction2 = Transaction::new(bhupendra.public_key, jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);
    println!(
        "Transaction 2 signature validation: {:#?}",
        transaction2.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction2);

    let mut transaction3 = Transaction::new(jash.public_key, utsav.public_key, 10.0, None);
    transaction3.sign_transaction(&jash);
    println!(
        "Transaction 3 signature validation: {:#?}",
        transaction3.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction3);

    let _last_block_hash = block0.calculate_hash();
    let mut coin_chain = Blockchain::new();
    coin_chain.add_block(block0);
    coin_chain.dump_blockchain();
}