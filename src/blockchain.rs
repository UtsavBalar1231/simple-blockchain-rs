use super::block::Block;

/// A blockchain is a collection of blocks.
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// This method creates a new blockchain.
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    /// This method adds a block to the blockchain.
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// This method prints the blockchain.
    pub fn dump_blockchain(&self) {
        println!("\nNumber of blocks in chain: {}", &self.blocks.len());
        for block in &self.blocks {
            println!("\n# block: {}\n", block.index);
            for t in &block.verified_transactions {
                t.print_transaction();
            }
        }
    }
}
