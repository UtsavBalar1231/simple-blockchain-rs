use clap::{App, Arg, SubCommand};
use secp256k1::PublicKey;
#[allow(unused_imports)]
use simple_blockchain_rs::{
    block::Block, blockchain::Blockchain, client::Client, tests::*, transaction::Transaction,
};
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
fn tests() {
    client_class_print_key();
    transaction_class_print_transaction();
    transaction_class_print_multiple_transactions();
    block_class_print_block();
    blockchain_class_print_blockchain();
    block_mine();
    push_block_into_blockchain();
    test_new_blockchain();
}

fn main() -> std::io::Result<()> {
    let mut blockchain = Blockchain::new();
    blockchain
        .start_blockchain()
        .expect("Blockchain failed to start");
    println!("Blockchain started");
    println!("Your public key is: {}", blockchain.client.identify());

    let _ = App::new("Simple Blockchain")
        .version("0.1.2")
        .author("Utsav Balar <utsavbalar1231@gmail.com>")
        .about("A simple blockchain implementation in Rust")
        .subcommand(
            SubCommand::with_name("pubkey").about("Create a new client and print its public key"),
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("Get the balance of a client")
                .arg(
                    Arg::with_name("public_key")
                        .help("The public key of the client")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("mine").about("Mine a block"))
        .subcommand(SubCommand::with_name("mempool").about("Print the transactions in the mempool"))
        .subcommand(
            SubCommand::with_name("send")
                .about("Create a new transaction and print it")
                .arg(
                    Arg::with_name("receiver")
                        .short('r')
                        .long("receiver")
                        .value_name("RECEIVER")
                        .help("The receiver of the transaction")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("amount")
                        .short('a')
                        .long("amount")
                        .value_name("AMOUNT")
                        .help("The amount of the transaction")
                        .takes_value(true)
                        .required(true),
                ),
        );

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let mut args = input.split_whitespace();

        let command = args.next().unwrap();
        match command {
            "pubkey" => {
                let client = Client::new();
                println!("client public key: {}", client.identify());
            }
            "balance" => {
                println!("{{");
                for (&pubkey, &amount) in &blockchain.balances {
                    println!("{}: {}", pubkey, amount);
                }
                println!("}}");
            }
            "mine" => {
                let block = blockchain.mine();
                println!("Block mined: {:#?}", block);
            }
            "mempool" => {
                println!("Mempool: {:#?}", blockchain.mempool);
            }
            "send" => {
                let receiver = args.next().unwrap();
                let amount = args.next().unwrap();
                let amount = f64::from_str(amount).unwrap();
                let client = Client::new();
                let mut transaction = Transaction::new(
                    Some(client.public_key),
                    PublicKey::from_str(receiver).unwrap(),
                    amount,
                    None,
                );
                transaction.sign_transaction(&client);
                transaction.print_transaction();
            }
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }

    Ok(())
}
