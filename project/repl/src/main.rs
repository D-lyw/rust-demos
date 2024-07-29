use clap::Command;
use reedline_repl_rs::{Repl, Result};
use repl::{get_callbacks, ReplCommand, ReplContext};

fn main() -> Result<()> {
    let callbacks = get_callbacks();
    let ctx = ReplContext::new();

    let mut repl = Repl::new(ctx)
        .with_name("MyRepl")
        .with_description("my repl demo tool")
        .with_derived::<ReplCommand>(callbacks);

    repl.run()
}


