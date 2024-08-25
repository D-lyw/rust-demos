use crate::blocks::block::Transaction;
use crate::blocks::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    height: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blocks = Vec::new();
        let genesis_block = Block::create_genesis_block();
        blocks.push(genesis_block);
        Blockchain { blocks, height: 0 }
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(prev_block.block_hash(), transactions, 0x1FFFFFF);
        new_block.mine();

        self.blocks.push(new_block);
        self.height += 1;
    }

    pub fn mock_mine_block(&mut self) {
        let prev_block = self.blocks.last().unwrap();
        let transactions: Vec<Transaction> = vec![];
        let mut new_block = Block::new(prev_block.block_hash(), transactions, 0);
        new_block.mine();
        self.blocks.push(new_block);
        self.height += 1;
    }

    /// display all block info
    pub fn block_info(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("Block {}: {:?}", i, block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_mock_mine_block() {
        let mut blockchain = Blockchain::new();
        blockchain.mock_mine_block();
        blockchain.mock_mine_block();
        blockchain.block_info();
    }

    #[test]
    fn test_blockchain_mine_block() {
        let mut blockchain = Blockchain::new();
        blockchain.mine_block(vec![]);
        blockchain.mine_block(vec![]);
        blockchain.mine_block(vec![]);
        blockchain.block_info();
    }
}
