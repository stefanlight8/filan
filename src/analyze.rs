use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time::Instant;

use crate::interface::SortField;
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

pub fn analyze(dir: PathBuf, sort_by: SortField, limit: usize) -> Result<(), Error> {
    let start = Instant::now();
    let mut file_types = get_files_types(dir).into_iter().collect::<Vec<_>>();
    let time_elapsed = Instant::now() - start;

    let mut total_file_count_all = 0;
    let mut total_file_size_all = 0;
    for (_, data) in &file_types {
        total_file_count_all += data.count;
        total_file_size_all += data.size;
    }

    match sort_by {
        SortField::Name => file_types.sort_by_key(|(ext, _)| ext.clone()),
        SortField::Count => file_types.sort_by_key(|(_, data)| Reverse(data.count)),
        SortField::Size => file_types.sort_by_key(|(_, data)| Reverse(data.size)),
    }

    if limit != 0 && limit < file_types.len() {
        file_types.truncate(limit);
    }

    let mut total_file_count_shown = 0;
    let mut total_file_size_shown = 0;
    for (_, data) in &file_types {
        total_file_count_shown += data.count;
        total_file_size_shown += data.size;
    }

    let mut max_extension_width = 4;
    let mut max_size_width = 4;

    for (extension, data) in &file_types {
        max_extension_width = max_extension_width.max(extension.len());
        max_size_width = max_size_width.max(humanize_bytes(data.size).len());
    }

    let separator = "-".repeat(max_extension_width + max_size_width + 14);

    println!(
        "{:<ext_width$} {:>size_width$} {:>10}\n{separator}",
        "Type",
        "Size",
        "Files",
        ext_width = max_extension_width,
        size_width = max_size_width
    );

    for (extension, data) in &file_types {
        println!(
            "{:<ext_width$} {:>size_width$} {:>10}",
            extension,
            humanize_bytes(data.size),
            data.count,
            ext_width = max_extension_width,
            size_width = max_size_width
        );
    }

    println!(
        "{separator}\nSummary shown: {:>size_width$} ({:>5} files)",
        humanize_bytes(total_file_size_shown),
        total_file_count_shown,
        size_width = max_size_width
    );

    if total_file_count_all != total_file_count_shown {
        println!(
            "Total analyzed: {:>size_width$} ({:>5} files)",
            humanize_bytes(total_file_size_all),
            total_file_count_all,
            size_width = max_size_width
        );
    }

    println!("Time elapsed: {}ms", time_elapsed.as_millis());

    Ok(())
}
