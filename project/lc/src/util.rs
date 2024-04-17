use std::{collections::HashMap, sync::MutexGuard};

use tabled::{
    settings::{
        format,
        object::{FirstRow, LastRow},
        Style,
    },
    Table,
};
use walkdir::DirEntry;

use crate::CodeLines;
use owo_colors::OwoColorize;

pub fn print_code_info(code_lines_map: MutexGuard<HashMap<String, CodeLines>>) {
    let mut counter_info = vec![];

    let mut total_lines_info = CodeLines::new("Total".to_string());

    let mut list = code_lines_map
        .iter()
        .map(|(_, val)| val)
        .collect::<Vec<_>>();

    list.sort_by(|a, b| b.total_lines.cmp(&a.total_lines));

    for value in list {
        counter_info.push(value);
        total_lines_info += value.clone();
    }
    counter_info.push(&total_lines_info);

    println!(
        "{}",
        Table::new(counter_info)
            .with(Style::modern_rounded())
            .modify(FirstRow, format::Format::content(|s| s.bold().to_string()))
            .modify(
                LastRow,
                format::Format::content(|s| s.red().bold().to_string())
            )
            .to_string()
    );
}

pub fn is_hidden_type_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

pub fn is_supported_file_type(entry: &DirEntry) -> bool {
    let kind = match infer::get_from_path(entry.path()) {
        Ok(Some(kind)) => kind,
        Ok(None) => {
            // infer crate not support this program language file type
            return true;
        },
        Err(_) => return false,
    };

    // current only support count Text file type
    if kind.matcher_type() == infer::MatcherType::Text {
        return true;
    }

    false
}
