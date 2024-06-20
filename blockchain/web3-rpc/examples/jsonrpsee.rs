use alloy::primitives::utils::{parse_ether, Unit};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::types::Params;
use revm::primitives::U256;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc_client = HttpClientBuilder::default().build("http://127.0.0.1:8545")?;

    let block_number: String = rpc_client.request("eth_blockNumber", rpc_params![]).await?;

    println!("BlockNumber: {:?}", block_number);

    let balance: String = rpc_client.request("eth_getBalance", rpc_params!["0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", "latest"]).await?;

    println!("Balance: {:?}", balance);

    let one_eth: U256 = parse_ether("1")?;
    assert_eq!(one_eth, Unit::ETHER.wei());

    Ok(())
}