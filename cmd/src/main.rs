use anyhow::Result;
use clap::Parser;
use cmd::{CliApp, CommandExecutor};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliApp::parse();

    cli.command.execute().await?;

    Ok(())
}
