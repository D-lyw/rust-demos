use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommands {
    #[command(about = "sign text")]
    Sign(SignOpts),
    #[command(about = "verify text")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, default_value = "blake3")]
    pub format: TextSignFormat,
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
