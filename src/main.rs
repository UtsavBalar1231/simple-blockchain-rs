#[allow(unused_imports)]
use simple_blockchain_rs::block::Block;
#[allow(unused_imports)]
use simple_blockchain_rs::blockchain::Blockchain;
use simple_blockchain_rs::client::Client;
use simple_blockchain_rs::transaction::Transaction;

fn client_class_print_key() {
    let client = Client::new();
    println!("client public key: {}", client.identify());
}

fn transaction_class_print_transaction() {
    let client = Client::new();
    let receiver = Client::new();
    let amount = 1.0;

    let mut transaction1 = Transaction::new(client.public_key, receiver.public_key, amount, None);
    transaction1.sign_transaction(&client);

    transaction1.print_transaction();

    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
}

fn transaction_class_print_multiple_transactions() {
    let mut transactions = vec![];
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

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
    transactions.push(transaction1);

    let mut transaction2 = Transaction::new(bhupendra.public_key, jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);
    println!(
        "Transaction 2 signature validation: {:#?}",
        transaction2.is_valid_transaction()
    );
    transactions.push(transaction2);

    let mut transaction3 = Transaction::new(jash.public_key, utsav.public_key, 10.0, None);
    transaction3.sign_transaction(&jash);
    println!(
        "Transaction 3 signature validation: {:#?}",
        transaction3.is_valid_transaction()
    );
    transactions.push(transaction3);

    println!("");
    transactions.iter().for_each(|transaction| {
        transaction.print_transaction();
    });
}

fn block_class_print_block() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let mut block0 = Block::genesis_block(&utsav);
    println!("Genesis Block: {:#?}", block0);

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

    println!("Block 0: {:#?}", block0);
}

fn blockchain_class_print_blockchain() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let mut block0 = Block::genesis_block(&utsav);

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

fn main() {
    client_class_print_key();
    transaction_class_print_transaction();
    transaction_class_print_multiple_transactions();
    block_class_print_block();
    blockchain_class_print_blockchain();
}
