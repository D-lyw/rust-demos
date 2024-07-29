use std::process;

use clap::ArgMatches;
use super::ReplCommandResult;

pub fn exit<T>(_: ArgMatches, _context: &mut T) -> ReplCommandResult {
    println!("Bye");
    process::exit(0);

    #[allow(unreachable_code)]
    Ok(None)
}