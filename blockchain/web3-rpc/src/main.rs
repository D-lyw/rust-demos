use alloy::{providers::Provider, sol};
use tokio;
use web3_rpc::{init_ws_rpc_server, subscribe_event_logs, subscribe_usdt_transfer};

// sol!(
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     IWETH,
//     "examples/abi/weth.json"
// );

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ws_server = init_ws_rpc_server().await?;
    println!("Latest block: {}", ws_server.get_block_number().await?);

    // subscribe_blocks(&ws_server).await?;
    // subscribe_event_logs(&ws_server).await?;
    subscribe_usdt_transfer(&ws_server).await?;

    // let contract = IWETH::new("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?, ws_server);
    // let IWETH::totalSupplyReturn { _0 } = contract.totalSupply().call().await?;
    // println!("WETH total supply: {:?}", _0);

    Ok(())
}
