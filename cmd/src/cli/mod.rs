use clap::Parser;
use enum_dispatch::enum_dispatch;

use self::{jwt::JwtSubComand, passgen::PassGenerateOpts, text::TextSubCommands};

pub mod jwt;
pub mod passgen;
pub mod text;

#[derive(Parser, Debug)]
#[command(version, about = "some command line utilities")]
pub struct CliApp {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum SubCommand {
    // password generate
    #[command(name = "passgen", about = "generate password")]
    PassGenerate(PassGenerateOpts),

    // sign and verify text
    #[command(subcommand, name = "text", about = "sign and verify text")]
    Text(TextSubCommands),

    // generate and verify jwt
    #[command(subcommand, name = "jwt", about = "generate and verify jwt")]
    Jwt(JwtSubComand),
}

// impl CommandExecutor for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::PassGenerate(opts) => opts.execute().await,
//             SubCommand::Text(text_subcommand) => match text_subcommand {
//                 TextSubCommands::Sign(sign_opts) => sign_opts.execute().await,
//                 TextSubCommands::Verify(verify_opts) => verify_opts.execute().await,
//             },
//         }
//     }
// }
