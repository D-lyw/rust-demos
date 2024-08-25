use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(prev_block_hash: [u8; 32], merkle_root: [u8; 32]) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("时间获取错误");
        BlockHeader {
            version: 1,
            prev_block_hash,
            merkle_root,
            timestamp: now.as_secs(),
            bits: 0x1FFFFFFF, // 难度目标，这里简化处理
            nonce: 0,
        }
    }

    pub fn set_nonce(&mut self, nonce: u32) {
        self.nonce = nonce;
    }

    pub fn set_bits(&mut self, bits: u32) {
        self.bits = bits;
    }

    pub fn block_hash(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.prev_block_hash);
        hasher.update(&self.merkle_root);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.bits.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>, // 假设我们有一个 Transaction 结构体
}

impl Block {
    pub fn new(prev_block_hash: [u8; 32], transactions: Vec<Transaction>, bits: u32) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions); // 假设我们有这个方法
        let mut header = BlockHeader::new(prev_block_hash, merkle_root);
        header.set_bits(bits);

        Block {
            header,
            transactions,
        }
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
        // 这里应该实现默克尔树根的计算
        // 简化处理，返回一个全零的哈希
        [0; 32]
    }

    pub fn block_hash(&self) -> [u8; 32] {
        self.header.block_hash()
    }

    pub fn create_genesis_block() -> Self {
        let genesis_transaction = Transaction {
            id: [0; 32],
            data: "创世区块交易".to_string(),
        };
        let transactions = vec![genesis_transaction];
        let prev_block_hash = [0; 32]; // 创世区块的前一个区块哈希为全零
        Self::new(prev_block_hash, transactions, 0)
    }

    pub fn mine(&mut self) {
        let target = self.calculate_target();
        println!("target: {:?}", target);
        loop {
            let hash = self.header.block_hash();
            if self.check_pow(&hash, target) {
                break;
            }
            self.header.nonce += 1;
        }
    }

    fn calculate_target(&self) -> u32 {
        // let mut target = [0xff; 32];
        // let exponent = self.header.bits >> 24;
        // let mantissa = self.header.bits & 0x00ffffff;
        // let shift = 8 * (32 - 3 - exponent as usize);

        // if shift < 32 {
        //     target[shift..].copy_from_slice(&mantissa.to_be_bytes()[1..]);
        // }

        // target

        // 简化目标难度
        self.header.bits
    }

    fn check_pow(&self, hash: &[u8; 32], target: u32) -> bool {
        // 将哈希的前4个字节转换为u32，并与目标比较
        let hash_value = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);
        hash_value < target
    }
}

// 为了完整性，我们还需要定义一个简单的 Transaction 结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    // 这里简化处理，实际的交易结构会更复杂
    pub id: [u8; 32],
    pub data: String,
}
