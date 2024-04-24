use base64::prelude::*;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{path::PathBuf, str::FromStr};

use crate::{get_key_content, get_reader, handle_text_sign, handle_text_verify, CommandExecutor};

#[derive(Debug, Parser)]
#[enum_dispatch(CommandExecutor)]
pub enum TextSubCommands {
    #[command(about = "sign text")]
    Sign(SignOpts),
    #[command(about = "verify text")]
    Verify(VerifyOpts),
}

// impl CommandExecutor for TextSubCommands {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             TextSubCommands::Sign(opts) => opts.execute().await,
//             TextSubCommands::Verify(opts) => opts.execute().await,
//         }
//     }
// }

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CommandExecutor for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key_content = get_key_content(&self.key)?;
        let signature = handle_text_sign(&mut reader, key_content, self.format)?;
        println!("{}", BASE64_STANDARD.encode(&signature));
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(long)]
    pub signature: String,
}

impl CommandExecutor for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key_content = get_key_content(&self.key)?;
        let result = handle_text_verify(
            &mut reader,
            key_content,
            self.format,
            BASE64_STANDARD.decode(&self.signature)?,
        )?;
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("invalid sign format: {}", s)),
        }
    }
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || PathBuf::from(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
