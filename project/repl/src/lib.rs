mod cli;

use std::collections::VecDeque;

pub use cli::exit;
pub use cli::ReplCommand;
use reedline_repl_rs::CallBackMap;

pub struct ReplContext {
    list: VecDeque<String>,
}
impl ReplContext {
    pub fn new() -> Self {
        ReplContext {
            list: VecDeque::new(),
        }
    }
}

pub type ReplCallbacks = CallBackMap<ReplContext, reedline_repl_rs::Error>;

pub fn get_callbacks() -> ReplCallbacks {
    let mut callbacks = CallBackMap::new();

    // add your callback functions here
    callbacks.insert("hello".to_string(), cli::hello);
    callbacks.insert("ls".to_string(), cli::ls);
    callbacks.insert("exit".to_string(), cli::exit);
    
    callbacks
}