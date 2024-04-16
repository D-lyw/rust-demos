pub mod util;
pub mod process;

use std::{
    collections::HashMap, fs::File, io::{self, BufRead, BufReader}, ops::AddAssign, path::Path, sync::{Arc, Mutex}, thread
};

use clap::Parser;
use hyperpolyglot;
use process::scan_dir_files;
use tabled::Tabled;

use crate::util::print_code_info;

#[derive(Parser, Debug)]
#[command(name = "lc", about = "line count for project codes")]
struct Args {
    #[arg(
        short,
        long = "dir",
        value_name = "path",
        help = "",
        default_value = "."
    )]
    dir: String,
}

#[derive(Tabled, Debug, Clone)]
#[tabled(rename_all = "PascalCase")]
pub struct CodeLines {
    language_type: String,
    total_lines: usize,
    code_lines: usize,
    comment_lines: usize,
    empty_lines: usize,
}

impl CodeLines {
    pub fn new(name: String) -> Self {
        CodeLines {
            language_type: name,
            total_lines: 0,
            code_lines: 0,
            comment_lines: 0,
            empty_lines: 0,
        }
    }
}

impl AddAssign for CodeLines {
    fn add_assign(&mut self, other: Self) {
        self.total_lines += other.total_lines;
        self.code_lines += other.code_lines;
        self.empty_lines += other.empty_lines;
        self.comment_lines += other.comment_lines;
    }
}

fn main() {
    let args = Args::parse();

    let mut handles = vec![];

    let files = scan_dir_files(&args.dir);

    let code_lines_map = Arc::new(Mutex::new(HashMap::new()));

    for file in files {
        let code_lines_map = Arc::clone(&code_lines_map);

        // TODO: set max thread number to control max opening files
        let handle = thread::spawn(move || {
            // check file code language type
            let language_type = match hyperpolyglot::detect(file.path()) {
                Ok(Some(detection)) => detection.language(),
                Ok(None) => "Unkown",
                Err(err) => {
                    // eprintln!("Detect Program Language Type Error: {:?}", err);
                    // eprintln!("Error Path: {:?}", file);
                    return;
                }
            };
            if let Ok(lines) = count_code_lines(file.path(), language_type) {

                let mut code_lines_map = match code_lines_map.lock() {
                    Ok(code_lines_map) => code_lines_map,
                    Err(err) => {
                        eprintln!("Lock Code Lines Map Error: {:?}", err);
                        std::process::exit(1);
                    }
                };

                let lines_clone = lines.clone();
                code_lines_map
                    .entry(language_type)
                    .and_modify(|counter| *counter += lines)
                    .or_insert(lines_clone);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let code_lines_map = match code_lines_map.lock() {
        Ok(code_lines_map) => code_lines_map,
        Err(err) => {
            eprintln!("Lock Code Lines Map Error: {:?}", err);
            std::process::exit(1);
        }
    };
    
    print_code_info(code_lines_map);
}



// count code lines
pub fn count_code_lines(path: &Path, language_type: &str) -> io::Result<CodeLines> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Opening file error: {:?}", err);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    let mut code_lines = CodeLines::new(language_type.to_string());

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Reading line error: {:?}", err);
                std::process::exit(1);
            }
        };

        // split command line and code lines
        code_lines.total_lines += 1;
        match line.trim() {
            l if l.is_empty() => code_lines.empty_lines += 1,
            l if l.starts_with("//") => code_lines.comment_lines += 1,
            _ => code_lines.code_lines += 1,
        }
    }

    Ok(code_lines)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}