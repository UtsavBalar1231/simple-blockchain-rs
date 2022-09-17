use crate::{block::*, client::*, transaction::*};
use std::collections::HashMap;

const DIFFICULTY_LEVEL: usize = 2;

/// A blockchain is a collection of blocks.
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub balances: HashMap<PublicKey, f64>,
    pub mempool: Vec<Transaction>,
    pub client: Client,
}

impl Blockchain {
    /// This method creates a new blockchain.
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            balances: HashMap::new(),
            mempool: Vec::new(),
            client: Client::new(),
        }
    }

    fn process_block_transactions(&mut self, block: &Block) {
        for (i, transaction) in block.verified_transactions.iter().enumerate() {
            // Process: Sender -> Receiver (Deduct amount from balance)
            if i > 0 { // Skip for coinbase transaction
                let sender_balance = self
                    .balances
                    .get_mut(&transaction.sender.unwrap())
                    .expect("Balance is empty.");
                *sender_balance -= transaction.amount;
            }
            // Process: Receiver <- Sender (Insert amount into balance)
            let receiver_balance = self.balances.entry(transaction.receiver).or_insert(0.0);
            *receiver_balance += transaction.amount;
        }
    }


    /// This method process a block in blockchain
    fn process_block(&mut self, block: &Block) -> Result<(), &'static str> {
        block.verify_block()?;
        self.process_block_transactions(&block);
        self.blocks.push(block.clone());
        return Ok(());
    }

    /// This method is used to start a new blockchain with genesis block included
    pub fn start_blockchain(&mut self) -> Result<Block, &'static str> {
        let genesis_block = Block::genesis_block(&self.client).mine_block(DIFFICULTY_LEVEL)?;
        println!("genesis block: {:#?}", genesis_block);
        self.process_block(&genesis_block)?;
        Ok(genesis_block)
    }

    /// This method prints the blockchain.
    pub fn dump_blockchain(&self) {
        println!("\nNumber of blocks in chain: {}", &self.blocks.len());
        for block in &self.blocks {
            println!("\n# block: {}\n", block.index);
            println!(
                "No. of transactions in block: {}",
                block.verified_transactions.len()
            );
            for t in &block.verified_transactions {
                t.print_transaction();
            }
        }
    }
}
