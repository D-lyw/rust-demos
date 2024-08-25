use crate::blocks::Block;
use rocksdb::DB;

pub const HEIGHT_KEY: &str = "height";

#[derive(Debug)]
pub struct Storage {
    db: DB,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(path).unwrap();
        let height: usize = db
            .get(HEIGHT_KEY)
            .unwrap()
            .map(|data| bincode::deserialize(&data).unwrap())
            .unwrap_or(0);
        Self { db }
    }

    pub fn add_block(&mut self, height: usize, block: &Block) {
        self.db
            .put(height.to_be_bytes(), bincode::serialize(block).unwrap())
            .unwrap();
        self.db
            .put(HEIGHT_KEY, bincode::serialize(&height).unwrap())
            .unwrap();
    }

    pub fn get_block(&self, height: usize) -> Option<Block> {
        self.db
            .get(height.to_be_bytes())
            .unwrap()
            .map(|data| bincode::deserialize(&data).unwrap())
    }

    pub fn get_height(&self) -> usize {
        self.db
            .get(HEIGHT_KEY)
            .unwrap()
            .map(|data| bincode::deserialize(&data).unwrap())
            .unwrap_or(0)
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = Block> + '_ {
        let height = self.get_height();
        (0..=height).filter_map(move |i| self.get_block(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let storage = Storage::new("test_db");
        println!("{:?}", storage);
    }
}
