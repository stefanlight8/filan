use std::collections::HashMap;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use crate::structs::FileTypeData;
use crate::utils::{humanize_bytes, walk_dir};

pub fn get_files_types(dir: PathBuf) -> HashMap<String, FileTypeData> {
    let mut types = HashMap::new();
    let files = walk_dir(dir).expect("Cannot get files from target directory");
    for file in files {
        if let Some(file_type) = file.extension().and_then(|ext| ext.to_str()) {
            let entry = types
                .entry(file_type.to_string())
                .or_insert(FileTypeData { count: 0, size: 0 });
            entry.count += 1;
            entry.size += file.metadata().map(|m| m.size() as usize).unwrap_or(0);
        }
    }

    types
}

pub fn analyze(dir: PathBuf) -> Result<(), Error> {  // TODO: Simplify
    let mut types_data: Vec<_> = get_files_types(dir).into_iter().collect();
    types_data.sort_by_key(|(_, data)| data.size);
    types_data.reverse();

    let (total_files, total_size) = types_data
        .iter()
        .fold((0, 0), |(files, size), (_, data)| {
        (files + data.count, size + data.size)
    });

    let max_ext_len = types_data
        .iter()
        .map(|(ext, _)| ext.len())
        .max()
        .unwrap_or(4);
    let max_size_len = types_data
        .iter()
        .map(|(_, data)| humanize_bytes(data.size).len())
        .max()
        .unwrap_or(4);

    let header = format!(
        "{:<ext_width$} {:>size_width$} {:>10}",
        "Type",
        "Size",
        "Files",
        ext_width = max_ext_len,
        size_width = max_size_len
    );
    let separator = "-".repeat(header.len());

    println!("{header}\n{separator}");
    for (ext, data) in &types_data {
        println!(
            "{:<ext_width$} {:>size_width$} {:>10}",
            ext,
            humanize_bytes(data.size),
            data.count,
            ext_width = max_ext_len,
            size_width = max_size_len
        );
    }
    println!(
        "{separator}\nSummary: {:>size_width$} ({:>5} files)",
        humanize_bytes(total_size),
        total_files,
        size_width = max_size_len
    );

    Ok(())
}
