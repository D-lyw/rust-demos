use walkdir::{DirEntry, WalkDir};

use crate::util::is_hidden_type_file;

// 递归遍历目录下所有文件
pub fn scan_dir_files(path: &str) -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = Vec::new();

    let default_exclude_list: Vec<&str> = vec![
        "Cargo.toml",
        "target",
        "debug",
        "build",
        "deps",
        "node_modules",
    ];

    let filter_handler = |entry: &DirEntry| {
        if is_hidden_type_file(entry) {
            return false;
        }
        if entry.file_name().to_string_lossy().starts_with('.') {
            return false;
        }
        if entry.file_type().is_dir() {}

        if let Some(file_name) = entry.file_name().to_str() {
            if default_exclude_list.contains(&file_name) {
                return false;
            }
        }

        true
    };

    // TODO: 添加过滤机制，控制包含或不包含哪类文件
    for entry in WalkDir::new(path).into_iter().filter_entry(filter_handler) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Scanning directory error: {:?}", err);
                continue;
            }
        };
        // in exclude directories, skip current directory

        if entry.file_type().is_file() {
            files.push(entry);
        }
    }

    files
}
