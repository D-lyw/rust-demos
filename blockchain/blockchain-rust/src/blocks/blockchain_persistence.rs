use crate::blocks::block::Transaction;
use crate::blocks::Block;
use crate::storage::Storage;

#[derive(Debug)]
pub struct Blockchain {
    storage: Storage,
    height: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut storage = Storage::new("blockchain_db");

        if storage.get_height() == 0 {
            let genesis_block = Block::create_genesis_block();
            storage.add_block(0, &genesis_block);
            Blockchain { storage, height: 0 }
        } else {
            let height = storage.get_height();
            Blockchain { storage, height }
        }
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self
            .storage
            .get_block(self.height)
            .expect("Failed to get prev block");
        let mut new_block = Block::new(prev_block.block_hash(), transactions, 0x1FFFFFF);
        new_block.mine();

        self.height += 1;
        self.storage.add_block(self.height, &new_block);
    }

    pub fn mock_mine_block(&mut self) {
        let prev_block = self
            .storage
            .get_block(self.height)
            .expect("Failed to get prev block");
        let transactions: Vec<Transaction> = vec![];
        let mut new_block = Block::new(prev_block.block_hash(), transactions, 0);
        new_block.mine();

        self.height += 1;
        self.storage.add_block(self.height, &new_block);
    }

    /// display all block info
    pub fn block_info(&self) {
        let block = self.storage.iter_blocks();
        for (index, block) in block.enumerate() {
            println!("Block {:?}: {:?}", index, block);
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
