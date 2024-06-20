use std::time::Duration;

use tokio::{self, time::sleep};
use web3::{self, types::{TransactionRequest, U256}};

#[tokio::main]
async fn main() -> web3::Result {
    let transport = web3::transports::Http::new("http://127.0.0.1:8545")?;
    let _web3 = web3::Web3::new(transport);

    let accounts = _web3.eth().accounts().await?;

    // get first and second account from account list
    let account1 = accounts.get(0).expect("Account not found");
    let account2 = accounts.get(1).expect("Account not found");

    println!("Account1 balance: {}", _web3.eth().balance(account1.to_owned(), None).await?);
    println!("Account2 balance: {}", _web3.eth().balance(account2.to_owned(), None).await?);

    let send_tx = TransactionRequest {
        from: account1.to_owned(),
        to: Some(account2.to_owned()),
        value: Some(U256::exp10(18).checked_mul(U256::from("2")).expect("Invalid value")),
        ..Default::default()
    };

    let result = _web3.eth().send_transaction(send_tx).await?;
    println!("Tx succeeded with hash: {}", result);

    let _ = sleep(Duration::from_secs(1));
    println!("Account1 balance: {}", _web3.eth().balance(account1.to_owned(), None).await?);
    println!("Account2 balance: {}", _web3.eth().balance(account2.to_owned(), None).await?);


    println!("Current block: {}", _web3.eth().block_number().await?);
    Ok(())
}