use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn dump_blockchain(&self) {
        println!("Number of blocks in chain: {}\n", &self.blocks.len());
        for block in &self.blocks {
            println!("# block: {}\n", block.index);
            for t in &block.verified_transactions {
                t.print_transaction();
                println!("");
            }
            println!("");
        }
    }
}
