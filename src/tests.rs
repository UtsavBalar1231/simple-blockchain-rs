use super::block::Block;
use super::blockchain::Blockchain;
use super::client::Client;
use super::transaction::Transaction;

#[allow(dead_code)]
pub fn client_class_print_key() {
    let client = Client::new();
    println!("client public key: {}", client.identify());
}

#[allow(dead_code)]
pub fn transaction_class_print_transaction() {
    let client = Client::new();
    let receiver = Client::new();
    let amount = 1.0;

    let mut transaction1 = Transaction::new(Some(client.public_key), receiver.public_key, amount, None);
    transaction1.sign_transaction(&client);

    transaction1.print_transaction();

    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
}

#[allow(dead_code)]
pub fn transaction_class_print_multiple_transactions() {
    let mut transactions = vec![];
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());
    println!("");

    let mut transaction1 =
        Transaction::new(Some(utsav.public_key), bhupendra.public_key, 10.0, None);
    transaction1.sign_transaction(&utsav);
    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
    transactions.push(transaction1);

    let mut transaction2 =
        Transaction::new(Some(bhupendra.public_key), jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);
    println!(
        "Transaction 2 signature validation: {:#?}",
        transaction2.is_valid_transaction()
    );
    transactions.push(transaction2);

    let mut transaction3 = Transaction::new(Some(jash.public_key), utsav.public_key, 10.0, None);
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

#[allow(dead_code)]
pub fn block_class_print_block() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let mut block0 = Block::genesis_block(&utsav);
    println!("Genesis Block: {:#?}", block0);

    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());
    println!("");

    let mut transaction1 =
        Transaction::new(Some(utsav.public_key), bhupendra.public_key, 10.0, None);
    transaction1.sign_transaction(&utsav);
    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction1);

    let mut transaction2 =
        Transaction::new(Some(bhupendra.public_key), jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);
    println!(
        "Transaction 2 signature validation: {:#?}",
        transaction2.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction2);

    let mut transaction3 = Transaction::new(Some(jash.public_key), utsav.public_key, 10.0, None);
    transaction3.sign_transaction(&jash);
    println!(
        "Transaction 3 signature validation: {:#?}",
        transaction3.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction3);

    println!("Block 0: {:#?}", block0);
}

#[allow(dead_code)]
pub fn blockchain_class_print_blockchain() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let mut block0 = Block::genesis_block(&utsav);

    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());
    println!("");

    let mut transaction1 =
        Transaction::new(Some(utsav.public_key), bhupendra.public_key, 10.0, None);
    transaction1.sign_transaction(&utsav);
    println!(
        "Transaction 1 signature validation: {:#?}",
        transaction1.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction1);

    let mut transaction2 =
        Transaction::new(Some(bhupendra.public_key), jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);
    println!(
        "Transaction 2 signature validation: {:#?}",
        transaction2.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction2);

    let mut transaction3 = Transaction::new(Some(jash.public_key), utsav.public_key, 10.0, None);
    transaction3.sign_transaction(&jash);
    println!(
        "Transaction 3 signature validation: {:#?}",
        transaction3.is_valid_transaction()
    );
    block0.verified_transactions.push(transaction3);

    let _last_block_hash = block0.calculate_hash();
    let mut coin_chain = Blockchain::new();
    coin_chain.blocks.push(block0);
    coin_chain.dump_blockchain();
}

#[allow(dead_code)]
pub fn block_mine() {
    let utsav = Client::new();
    let mut block0 = Block::genesis_block(&utsav);
    block0 = block0.mine_block(3).unwrap();
    println!("{:#?}", block0);
}

#[allow(dead_code)]
pub fn push_block_into_blockchain() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    let mut block0 = Block::genesis_block(&utsav);
    block0 = block0.mine_block(2).unwrap();

    let mut block1 = Block::new(1, &block0.block_hash);
    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());
    println!("");

    let mut transaction1 =
        Transaction::new(Some(utsav.public_key), bhupendra.public_key, 10.0, None);
    transaction1.sign_transaction(&utsav);

    let mut transaction2 =
        Transaction::new(Some(bhupendra.public_key), jash.public_key, 10.0, None);
    transaction2.sign_transaction(&bhupendra);

    let mut transaction3 = Transaction::new(Some(jash.public_key), utsav.public_key, 10.0, None);
    transaction3.sign_transaction(&jash);

    block1.verified_transactions.push(transaction1);
    block1.verified_transactions.push(transaction2);
    block1.verified_transactions.push(transaction3);

    block1 = block1.mine_block(2).unwrap();

    let mut balar_chain = Blockchain::new();
    balar_chain.blocks.push(block0);
    balar_chain.blocks.push(block1);
    balar_chain.dump_blockchain();
}

#[allow(dead_code)]
pub fn test_new_blockchain() {
    let utsav = Client::new();
    let bhupendra = Client::new();
    let jash = Client::new();

    println!("utsav public key: {}", utsav.identify());
    println!("bhupendra public key: {}", bhupendra.identify());
    println!("jash public key: {}", jash.identify());

    let mut transaction0 =
        Transaction::new(Some(utsav.public_key), bhupendra.public_key, 10.0, None);
    transaction0.sign_transaction(&utsav);
    let mut transaction1 =
        Transaction::new(Some(bhupendra.public_key), jash.public_key, 10.0, None);
    transaction1.sign_transaction(&bhupendra);
    let mut transaction2 = Transaction::new(Some(jash.public_key), utsav.public_key, 10.0, None);
    transaction2.sign_transaction(&jash);

    let mut blockchain = Blockchain::new();
    blockchain.start_blockchain().unwrap();

    blockchain.mempool.push(transaction0);
    blockchain.mempool.push(transaction1);
    blockchain.mempool.push(transaction2);

    blockchain.mine().unwrap();

    blockchain.dump_blockchain();
}