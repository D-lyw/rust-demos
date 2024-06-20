use revm::primitives::bitvec::view::AsBits;
use web3::types::{H160, U256};

fn main() {
    let a = U256::from(12345678);
    // let mut arr: [usize; 4] = [12usize, 34usize, 56usize, 78usize];
    let mut arr = [0u8; 32];
    arr[31] = 0;
    arr[30] = 15;
    arr[29] = 1;
    arr[28] = 0;
    arr[27] = 10;
    let b = U256::from_little_endian(&arr);
    println!("{:?}", a.0);
    println!("{:?}", b.0);

    let mut buf = [0u8; 4];
    buf[0] = 0x12;
    buf[1] = 0x34;
    buf[2] = 0x56;
    buf[3] = 0x78;
    println!("{:?}", u32::from_be_bytes(buf));
    println!("{:?}", u32::from_le_bytes(buf));
    
    println!("{:x}", u32::from_be_bytes(buf));
    println!("{:x}", u32::from_le_bytes(buf));
}