mod hello;

pub use self::hello::hello;

use clap::Parser;

type ReplCommandResult = Result<Option<String>, reedline_repl_rs::Error>;

#[derive(Debug, Parser)]
#[command(name = "Repl", about = "Repl tools")]
pub enum ReplCommand {
    Hello { who: String },
}
