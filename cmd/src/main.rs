use anyhow::Result;
use clap::Parser;
use cmd::{get_reader, handle_text_sign, password_generate, CliApp};
use base64::prelude::*;

fn main() -> Result<()> {
    let cli = CliApp::parse();

    match cli.command {
        cmd::SubCommand::PassGenerate(options) => {
            let pass = password_generate(
                options.length,
                options.number,
                options.uppercase,
                options.lowercase,
                options.symbol,
            )?;
            println!("{}", pass);
        }
        cmd::SubCommand::Text(text_subcommand) => match text_subcommand {
            cmd::text::TextSubCommands::Sign(sign_opts) => {
                // handle input from stdin or file
                let mut reader = get_reader(&sign_opts.input)?;
                // get key content
                let key_content = cmd::get_key_content(&sign_opts.key)?;
                // handle text sign
                let signature = handle_text_sign(&mut reader, key_content, sign_opts.format)?;
                println!("{}", BASE64_STANDARD.encode(&signature));
            }
            cmd::text::TextSubCommands::Verify(verify_opts) => {}
        },
    }

    Ok(())
}
