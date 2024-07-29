use std::process;

use clap::ArgMatches;

use super::ReplCommandResult;

pub fn ls<T>(args: ArgMatches, _context: &mut T) -> ReplCommandResult {
    let mut command_args: Vec<String> = vec![];
    let show_detail = args.get_one::<bool>("list_detail");
    if let Some(true) = show_detail {
        command_args.push(format!("-l"));
    }
    let all_files = args.get_one::<bool>("all_files");
    if let Some(true) = all_files {
        command_args.push(format!("-a"));
    }
    let output = process::Command::new("ls")
        .args(command_args)
        .output()
        .expect("Command execute failed");

    if output.status.success() {
        let out_str = output.stdout;
        Ok(Some(
            String::from_utf8(out_str).expect("parser output failed"),
        ))
    } else {
        Err(reedline_repl_rs::Error::UnknownCommand(
            "Command execute failed".to_string(),
        ))
    }
}
