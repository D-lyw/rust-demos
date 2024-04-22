mod password_generate;
use clap::{value_parser, Parser, Subcommand};

use password_generate::password_generate;

#[derive(Parser, Debug)]
#[command(version, about = "some command line utilities")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // password generate
    #[command(name = "pass-generate", about = "generate password")]
    PassGenerate(PassGenerateOption),
}

#[derive(Parser, Debug)]
struct PassGenerateOption {
    #[arg(short, long, default_value_t = 16, value_parser = value_parser!(u8).range(6..))]
    length: u8,
    #[arg(long, default_value_t = true)]
    number: bool,
    #[arg(long, default_value_t = true)]
    lowercase: bool,
    #[arg(long, default_value_t = true)]
    symbol: bool,
    #[arg(long, default_value_t = true)]
    uppercase: bool,
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::PassGenerate(options)) => {
            password_generate(
                options.length,
                options.number,
                options.lowercase,
                options.symbol,
                options.uppercase,
            );
        }
        _ => {
            println!("other");
        }
    }
}
