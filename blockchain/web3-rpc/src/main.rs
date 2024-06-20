// use revm::{ db::{CacheDB, EmptyDB}, Evm};

// fn main() {
//     // create a blockchain transaction and execute by revm 

//     let mut cache_db = CacheDB::new(EmptyDB::default());


//     let mut evm = Evm::builder().build();

//     // evm.transact()

// }

use tokio;
use web3;

#[tokio::main]
async fn main() -> web3::Result {
    // let transport = web3::transports::Http::new("https://sepolia.infura.io/v3/2e1e3366832e41368179dc9e08156d85")?;
    
    let transport = web3::transports::Http::new("http://127.0.0.1:8545")?;
    let _web3 = web3::Web3::new(transport);

    let accounts = _web3.eth().accounts().await?;
    for _account in accounts {
        println!("Account: {:?}", _account);
    }

    println!("Current block: {}", _web3.eth().block_number().await?);
    Ok(())
}