use std::str::FromStr;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{
        address,
        utils::{format_units, parse_units},
    },
    providers::{Provider, ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{Filter, Log},
    transports::{RpcError, TransportErrorKind},
};
use anyhow;
use futures_util::StreamExt;
use revm::primitives::{uint, Address, U256};

pub async fn init_ws_rpc_server(
) -> anyhow::Result<RootProvider<PubSubFrontend>, RpcError<TransportErrorKind>> {
    let rpc_url = "wss://mainnet.infura.io/ws/v3/2e1e3366832e41368179dc9e08156d85";
    let ws = WsConnect::new(rpc_url);

    ProviderBuilder::new().on_ws(ws).await
}

pub async fn subscribe_blocks(server: &RootProvider<PubSubFrontend>) -> anyhow::Result<()> {
    let sub = server.subscribe_blocks().await?;
    let mut block_stream = sub.into_stream();
    while let Some(block) = block_stream.next().await {
        println!("Block: {:?}", block.header);
    }
    Ok(())
}

/// subscribe smart contract function call event log
pub async fn subscribe_event_logs(server: &RootProvider<PubSubFrontend>) -> anyhow::Result<()> {
    let weth_address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    let filter = Filter::new()
        .address(weth_address)
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    let sub = server.subscribe_logs(&filter).await?;

    let mut log_stream = sub.into_stream();
    while let Some::<Log>(log) = log_stream.next().await {
        println!("Contract log: {:?}, {:?}", log.block_hash, log.inner.data);
    }
    Ok(())
}

/// subscribe usdt transfer event
pub async fn subscribe_usdt_transfer(server: &RootProvider<PubSubFrontend>) -> anyhow::Result<()> {
    let usdt_address = address!("dac17f958d2ee523a2206206994597c13d831ec7");
    let filter = Filter::new()
        .address(usdt_address)
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    let sub = server.subscribe_logs(&filter).await?;

    let mut log_stream = sub.into_stream();
    while let Some::<Log>(log) = log_stream.next().await {
        let log_data = log.inner.data;
        let transfer_amount = U256::from_str(&log_data.data.to_string())?;
        let transfer_from = log_data.topics()[1];
        let transfer_to = log_data.topics()[2];
        let min_transfer_amount: U256 = parse_units("1000", 6)?.into();

        if transfer_amount >= min_transfer_amount {
            println!(
                "Transfer {:?} USDT from: {:?} to {:?}",
                format_units(transfer_amount, 6)?,
                Address::from_word(transfer_from),
                Address::from_word(transfer_to)
            );
        }
    }
    Ok(())
}
