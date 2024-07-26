use reedline_repl_rs::{ Repl, Result};
use repl::{get_callbacks, ReplCommand};

fn main() -> Result<()> {
    let callbacks = get_callbacks();

    let mut repl: Repl<(), reedline_repl_rs::Error> = Repl::new(())
        .with_name("MyRepl")
        .with_description("my repl demo tool")
        .with_derived::<ReplCommand>(callbacks);

    repl.run()
}


