use clap::Parser;

use self::{passgen::PassGenerateOpts, text::TextSubCommands};

pub mod passgen;
pub mod text;

#[derive(Parser, Debug)]
#[command(version, about = "some command line utilities")]
pub struct CliApp {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    // password generate
    #[command(name = "passgen", about = "generate password")]
    PassGenerate(PassGenerateOpts),

    // sign and verify text
    #[command(subcommand, name = "text", about = "sign and verify text")]
    Text(TextSubCommands),
}
