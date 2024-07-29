mod hello;
mod ls;
mod exit;

pub use self::exit::exit;
pub use self::hello::hello;
pub use self::ls::ls;

use clap::{Args, Parser};

type ReplCommandResult = Result<Option<String>, reedline_repl_rs::Error>;

#[derive(Debug, Parser)]
#[command(name = "Repl", about = "Repl tools")]
pub enum ReplCommand {
    Hello { who: String },
    LS(LsOptions),
    Exit
}

#[derive(Args, Debug)]
pub struct LsOptions {
    #[arg(long, default_value = ".")]
    pub dir: String,
    #[arg(short, default_value = "false")]
    pub list_detail: bool,
    #[arg(short, default_value = "false")]
    pub all_files: bool
}