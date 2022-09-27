use crate::{block::*, client::*, storage, transaction::*};
use rocksdb::DB;
use std::{fs::create_dir_all, path::Path};

const DIFFICULTY_LEVEL: usize = 2;
const BLOCKS_DB_PATH: &'static str = "db/_blocks";
const BLOCKS_METADATA_DB_PATH: &'static str = "db/_blocks_metadata";
const BALANCES_DB_PATH: &'static str = "db/_balances";
const WALLET_PATH: &'static str = "db/_wallets";
const DATA_DIR: &'static str = "db";

/// A blockchain is a collection of blocks.
pub struct Blockchain {
    pub blocks: DB,
    pub block_metadata: DB,
    pub balances: DB,
    pub mempool: Vec<Transaction>,
    pub client: Client,
}

impl Blockchain {
    /// This method creates a new blockchain.
    pub fn new() -> Self {
        create_dir_all(DATA_DIR).unwrap();

        Blockchain {
            blocks: DB::open_default(BLOCKS_DB_PATH).unwrap(),
            block_metadata: DB::open_default(BLOCKS_METADATA_DB_PATH).unwrap(),
            balances: DB::open_default(BALANCES_DB_PATH).unwrap(),
            mempool: Vec::new(),
            client: Blockchain::create_client().expect("Failed to create client"),
        }
    }

    pub fn create_client() -> Result<Client, &'static str> {
        if Path::new(WALLET_PATH).exists() {
            let key = std::fs::read_to_string(WALLET_PATH).unwrap();
            Ok(Client::from(key).unwrap())
        } else {
            let client = Client::new();
            std::fs::write(WALLET_PATH, client.identify()).unwrap();
            Ok(client)
        }
    }

    pub fn coinbase_transaction(&self) -> Transaction {
        Transaction::signed_new(
            &self.client,
            self.client.public_key,
            Block::get_block_reward(
                storage::get_latest_block_number(&self.block_metadata).unwrap() + 1,
            ),
        )
    }

    /// This method generates genesis block.
    pub fn genesis_block(&self) -> Block {
        let mut genesis_block = Block::new(0, String::from(GENESIS_BLOCK_HASH));

        genesis_block
            .verified_transactions
            .push(self.coinbase_transaction());

        genesis_block.block_hash = String::from(GENESIS_BLOCK_HASH);
        genesis_block
    }

    fn get_latest_block(&self) -> Result<Option<Block>, &'static str> {
        let block_hash = match storage::get_latest_block_hash(&self.block_metadata)? {
            Some(block_hash) => block_hash,
            None => return Ok(None),
        };
        let block = storage::get_block(&self.blocks, &block_hash)?;
        return Ok(block);
    }

    fn verify_transaction(&self, transaction: &Transaction) -> Result<(), &'static str> {
        transaction.is_valid_transaction()?;

        let balance = storage::get_balance(&self.balances, transaction.sender.unwrap())?;
        if balance.unwrap_or(0.0) < transaction.amount {
            return Err("Transaction verification failed: Insufficient funds");
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

    fn process_block_transactions(&mut self, block: &Block) -> Result<(), &'static str> {
        for (i, transaction) in block.verified_transactions.iter().enumerate() {
            let empty: f64 = 0.0;
            // i = 0 => Skip when coinbase transaction
            // Process: Sender => Receiver (Deduct amount from balance)
            if i > 0 {
                if let Some(sender_balance) =
                    storage::get_balance(&self.balances, transaction.sender.unwrap())
                        .unwrap_or(Some(empty))
                {
                    storage::set_balance(
                        &self.balances,
                        transaction.sender.unwrap(),
                        sender_balance - transaction.amount,
                    )?;
                } else {
                    return Err("Transaction verification failed: Sender balance not found");
                }
            }

            // Process: Receiver <= Sender (Insert amount into balance)
            if let Some(receiver_balance) =
                storage::get_balance(&self.balances, transaction.receiver).unwrap_or(Some(empty))
            {
                storage::set_balance(
                    &self.balances,
                    transaction.receiver,
                    receiver_balance + transaction.amount,
                )?;
            } else {
                storage::set_balance(&self.balances, transaction.receiver, transaction.amount)?;
            }
        }

        Ok(())
    }

    fn validate_last_block(&self, block: &Block) -> Result<(), &'static str> {
        let previous_block = self.get_latest_block()?;
        let previous_block_hash =
            previous_block.map_or(GENESIS_BLOCK_HASH.to_string(), |b| b.block_hash.clone());

        if block.previous_block_hash != previous_block_hash {
            return Err("Block verification: Must reference previous block's hash");
        }

        Ok(())
    }

    /// This method process a block in blockchain
    fn process_block(&mut self, block: &Block) -> Result<(), &'static str> {
        block.verify_block()?;
        self.validate_last_block(&block)?;
        self.process_block_transactions(&block)?;

        let previous_block_number = storage::get_latest_block_number(&self.block_metadata)?;
        storage::add_block(&self.blocks, block)?;
        storage::set_latest_block(
            &self.block_metadata,
            &block.block_hash,
            previous_block_number + 1,
        )?;

        return Ok(());
    }

    /// This method is used to start a new blockchain with genesis block included
    pub fn start_blockchain(&mut self) -> Result<Block, &'static str> {
        if let Ok(Some(latest_block)) = self.get_latest_block().map_err(|e| e) {
            return Ok(latest_block);
        } else {
            let genesis_block = self
                .genesis_block()
                .mine_block(DIFFICULTY_LEVEL)
                .expect("Failed to mine genesis block");
            self.process_block(&genesis_block)?;

            return Ok(genesis_block);
        }
    }

    pub fn mine(&mut self) -> Result<Block, &'static str> {
        let mut transactions = Vec::new();
        transactions.push(self.coinbase_transaction());
        transactions.extend(self.mempool.clone());

        let last_block = self.get_latest_block()?.unwrap();
        let mut new_block = Block::new(last_block.index + 1, last_block.block_hash.clone());
        new_block.verified_transactions = transactions;

        let mined_block = new_block.mine_block(DIFFICULTY_LEVEL)?;
        self.process_block(&mined_block)?;
        self.mempool.clear();

        Ok(mined_block)
    }
}
