use alloy::{network::TransactionBuilder, node_bindings::Anvil, primitives::utils::{format_ether, parse_ether}, providers::{Provider, ProviderBuilder}, rpc::types::TransactionRequest, sol, transports::http::reqwest};

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc a.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let anvil = Anvil::new().try_spawn()?;

    // println!("{:?}", anvil.keys());

    let local_rpc_url: reqwest::Url  = "http://localhost:8545".parse()?;

    let provider = ProviderBuilder::new().on_http(local_rpc_url);

    println!("Latest block: {}", provider.get_block_number().await?);
    println!("Accounts: {:?}", provider.get_accounts().await?);

    let accounts = provider.get_accounts().await?;

    let tx = TransactionRequest::default().with_to(accounts[1]).with_value(parse_ether("1")?);

    let pending_tx = provider.send_transaction(tx).await?;

    let receipt = pending_tx.get_receipt().await?;

    let new_balance = provider.get_balance(accounts[1]).await?;
    println!("Balance: {:?}", format_ether(new_balance));

    Ok(())
}