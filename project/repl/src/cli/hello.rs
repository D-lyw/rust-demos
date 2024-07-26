use clap::ArgMatches;
use super::ReplCommandResult;

// command hello
pub fn hello<T>(args: ArgMatches, _context: &mut T) -> ReplCommandResult {
    Ok(Some(format!("hello, welcome to use my repl")))
}