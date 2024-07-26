mod cli;

pub use cli::ReplCommand;
use reedline_repl_rs::CallBackMap;

pub type ReplCallbacks = CallBackMap<(), reedline_repl_rs::Error>;

pub fn get_callbacks() -> ReplCallbacks {
    let mut callbacks = CallBackMap::new();

    // add your callback functions here
    callbacks.insert("hello".to_string(), cli::hello);
    callbacks
}