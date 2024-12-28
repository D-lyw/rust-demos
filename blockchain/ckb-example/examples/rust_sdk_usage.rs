use std::{str::FromStr, u32};

use anyhow::anyhow;
use ckb_sdk::{
    rpc::{
        ckb_indexer::{Cell, Order, SearchKey},
        CkbRpcClient,
    },
    traits::CellQueryOptions,
    Address, NetworkType,
};
use ckb_types::{
    bytes::Bytes,
    packed::{Byte32, Script},
    prelude::*,
};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut ckb_client = CkbRpcClient::new("https://testnet.ckb.dev");

    let block = ckb_client.get_block_by_number(0.into())?;

    // println!("block: {}", serde_json::to_string_pretty(&block).unwrap());

    // let addr_str = "ckb1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqgvf0k9sc40s3azmpfvhyuudhahpsj72tsr8cx3d";
    let addr_str = "ckt1qrfrwcdnvssswdwpn3s9v8fp87emat306ctjwsm3nmlkjg8qyza2cqgqq94v6yy26rhlxhddveg4g5qt3u3h45la5sk73h7x";

    let addr_lock_script = parse_lock_script(addr_str)?;
    let query = CellQueryOptions::new(addr_lock_script, ckb_sdk::traits::PrimaryScriptType::Lock);

    let search_key = SearchKey::from(query);

    let cells = ckb_client.get_cells(search_key, Order::Desc, u32::MAX.into(), None)?;

    for cell in cells.objects {
        parse_print_cell(cell);
    }

    Ok(())
}

/// Parse lock script by address
fn parse_lock_script(address: &str) -> anyhow::Result<Script> {
    let addr = Address::from_str(address).map_err(|e| anyhow!(e))?;
    let addr_payload = addr.payload();

    let script = Script::new_builder()
        .code_hash(
            Byte32::from_slice(
                &addr_payload
                    .code_hash(Some(NetworkType::Testnet))
                    .as_bytes(),
            )
            .unwrap(),
        )
        .hash_type(addr_payload.hash_type().into())
        .args(Bytes::from(addr_payload.args()).pack())
        .build();

    Ok(script)
}

/// parse and print detail cell information
fn parse_print_cell(cell: Cell) {
    println!(
        "Cell OutPoint: {:?}",
        serde_json::to_string(&cell.out_point).unwrap()
    );

    println!("Cell Output: {:?}", cell.output);
    println!("Cell Capacity: {:?}", cell.output.capacity);
    println!(
        "Cell Lock Script: {:?}",
        serde_json::to_value(cell.output.lock.args).unwrap()
    );

    println!("--------------------");
}
