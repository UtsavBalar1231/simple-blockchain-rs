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

    pub fn coinbase_transaction(&self) -> Transaction {
        Transaction::signed_new(
            &self.client,
            self.client.public_key,
            Block::get_block_reward(self.blocks.len()),
        )
    }

    /// This method generates genesis block.
    pub fn genesis_block(&self) -> Block {
        let mut genesis_block = Block::new(0, GENESIS_BLOCK_HASH);

        genesis_block
            .verified_transactions
            .push(self.coinbase_transaction());

        genesis_block.block_hash = GENESIS_BLOCK_HASH;
        genesis_block
    }

    fn verify_transaction(&self, transaction: &Transaction) -> Result<(), &'static str> {
        transaction.is_valid_transaction()?;
        if transaction.amount <= 0.0 {
            return Err("Transaction verification failed: Amount is not valid");
        }

        if !self.balances.contains_key(&transaction.sender.unwrap()) {
            return Err("Transaction verification failed: Sender does not exist");
        }

        if self.balances[&transaction.sender.unwrap()] < transaction.amount {
            return Err("Transaction verification failed: Sender does not have enough balance");
        }

        Ok(())
    }

    pub fn send_transaction(
        &mut self,
        public_key: PublicKey,
        amount: f64,
    ) -> Result<Transaction, &'static str> {
        let tx = Transaction::signed_new(&self.client, public_key, amount);
        self.verify_transaction(&tx)?;
        self.mempool.push(tx.clone());
        return Ok(tx);
    }

    fn process_block_transactions(&mut self, block: &Block) {
        for (i, transaction) in block.verified_transactions.iter().enumerate() {
            let mut empty: f64 = 0.0;
            // i = 0 => Skip when coinbase transaction
            // Process: Sender => Receiver (Deduct amount from balance)
            if i > 0 {
                let sender_balance = self
                    .balances
                    .get_mut(&transaction.sender.unwrap())
                    .unwrap_or(&mut empty);
                *sender_balance -= transaction.amount;
            }
            // Process: Receiver <= Sender (Insert amount into balance)
            let receiver_balance = self.balances.entry(transaction.receiver).or_insert(0.0);
            *receiver_balance += transaction.amount;
        }
    }

    fn validate_last_block(&self, block: &Block) -> Result<(), &'static str> {
        let previous_block = self.blocks.last();
        let previous_block_hash =
            previous_block.map_or(super::block::GENESIS_BLOCK_HASH, |b| b.block_hash);

        if block.previous_block_hash != previous_block_hash.clone() {
            return Err("Block verification: Must reference previous block's hash");
        }

        Ok(())
    }

    /// This method process a block in blockchain
    fn process_block(&mut self, block: &Block) -> Result<(), &'static str> {
        block.verify_block()?;
        self.validate_last_block(&block)?;
        self.process_block_transactions(&block);
        self.blocks.push(block.clone());
        return Ok(());
    }

    /// This method is used to start a new blockchain with genesis block included
    pub fn start_blockchain(&mut self) -> Result<Block, &'static str> {
        let genesis_block = self
            .genesis_block()
            .mine_block(DIFFICULTY_LEVEL)
            .expect("Failed to mine genesis block");
        self.process_block(&genesis_block)?;

        Ok(genesis_block)
    }

    pub fn mine(&mut self) -> Result<Block, &'static str> {
        let mut transactions = Vec::new();
        transactions.push(self.coinbase_transaction());
        transactions.extend(self.mempool.clone());

        let last_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(last_block.index + 1, &last_block.block_hash);
        new_block.verified_transactions = transactions;

        let mined_block = new_block.mine_block(DIFFICULTY_LEVEL)?;
        self.process_block(&mined_block)?;
        self.mempool.clear();

        Ok(mined_block)
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
