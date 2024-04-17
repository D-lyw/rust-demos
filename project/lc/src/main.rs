pub mod process;
pub mod util;

use std::{
    collections::HashMap,
    ops::AddAssign,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use util::is_supported_file_type;
use walkdir::DirEntry;

use clap::Parser;
use hyperpolyglot;
use process::{count_code_lines, scan_dir_files};
use tabled::Tabled;
use tokio::sync::Semaphore;

use crate::util::print_code_info;

type CodeLinesMap = Arc<Mutex<HashMap<String, CodeLines>>>;

#[derive(Parser, Debug)]
#[command(
    name = "lc",
    about = "line counts by program language type for project codes",
    version = "0.1.0"
)]
struct Args {
    #[arg(
        short,
        long = "dir",
        value_name = "path",
        help = "",
        default_value = "."
    )]
    dir: String,
    #[arg(
        short,
        long,
        value_name = "excluded",
        help = "files and directorys that not count code lines",
        default_value = "[]"
    )]
    excluded: Vec<PathBuf>,
    #[arg(
        short,
        long,
        value_name = "max_depth",
        help = "max directory to scan",
        default_value = "10"
    )]
    limit_depth: usize,
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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let files = scan_dir_files(&args.dir, args.excluded, args.limit_depth);
    
    let code_lines_map = Arc::new(Mutex::new(HashMap::new()));

    // Control the maximum number of concurrently opened files by semaphore
    let max_file_open_size = 200;
    let semaphore = Arc::new(Semaphore::new(max_file_open_size));

    for file in files {
        handle_file(code_lines_map.clone(), semaphore.clone(), file).await;
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

pub async fn handle_file(code_lines_map: CodeLinesMap, semaphore: Arc<Semaphore>, file: DirEntry) {
    tokio::spawn(async move {
        // handle exclude binary, image... file types [Not UTF-8 encoded file types]
        if !is_supported_file_type(&file) {
            return;
        }
        // check file code language type
        let language_type = match hyperpolyglot::detect(&file.path()) {
            Ok(Some(detection)) => detection.language().to_string(),
            Ok(None) => "Unkown".to_string(),
            Err(_) => return,
        };

        let _premit_open_file = semaphore.acquire_owned().await.unwrap();

        if let Ok(lines) = count_code_lines(&file.path(), &language_type) {
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
    })
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
