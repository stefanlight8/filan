use std::collections::HashMap;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::Instant;

use crate::utils::{humanize_bytes, walk_dir};

#[derive(Debug, Clone)]
pub struct FileTypeData {
    pub count: usize,
    pub size: usize,
}

pub fn get_files_types(dir: PathBuf) -> HashMap<String, FileTypeData> {
    let mut types = HashMap::new();
    let files = walk_dir(dir).expect("Cannot get files from target directory");
    for file in files {
        if let Some(file_type) = file.extension().and_then(|ext| ext.to_str()) {
            let entry = types
                .entry(file_type.to_string().to_lowercase())
                .or_insert(FileTypeData { count: 0, size: 0 });
            entry.count += 1;
            entry.size += file.metadata().map(|m| m.size() as usize).unwrap_or(0);
        }
    }

    types
}

pub fn analyze(dir: PathBuf) -> Result<(), Error> {
    let start = Instant::now();
    let mut types_data = get_files_types(dir).into_iter().collect::<Vec<_>>();
    let time_elapsed = start.elapsed().as_millis();

    types_data.sort_by_key(|(_, data)| std::cmp::Reverse(data.size));

    let (total_files, total_size) = types_data.iter().fold((0, 0), |(files, size), (_, data)| {
        (files + data.count, size + data.size)
    });

    let (max_ext_len, max_size_len) =
        types_data
            .iter()
            .fold((4, 4), |(max_ext, max_size), (ext, data)| {
                (
                    max_ext.max(ext.len()),
                    max_size.max(humanize_bytes(data.size).len()),
                )
            });

    let separator = "-".repeat(max_ext_len + max_size_len + 14);

    println!(
        "{:<ext_width$} {:>size_width$} {:>10}\n{separator}",
        "Type",
        "Size",
        "Files",
        ext_width = max_ext_len,
        size_width = max_size_len
    );

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
        "{separator}\nSummary: {:>size_width$} ({:>5} files, time elapsed: {}ms)",
        humanize_bytes(total_size),
        total_files,
        time_elapsed,
        size_width = max_size_len
    );

    Ok(())
}
