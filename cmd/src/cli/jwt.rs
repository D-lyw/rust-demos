use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{handle_jwt_generate, handle_jwt_verify, CommandExecutor};

#[derive(Debug, Parser)]
#[enum_dispatch(CommandExecutor)]
pub enum JwtSubComand {
    #[command(about = "generate jwt token")]
    Sign(JwtSignOpts),
    #[command(about = "verify jwt token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short, long)]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub value: String,
}

impl CommandExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = handle_jwt_generate(self.sub, self.aud, self.exp)?;
        println!("{}", String::from_utf8(token)?);
        Ok(())
    }
}

impl CommandExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token_data = handle_jwt_verify(self.value)?;
        println!("Header: {:?}", token_data.header);
        println!("Payload: {:?}", token_data.claims);
        Ok(())
    }
}
