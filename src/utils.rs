use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;

pub fn walk_dir(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut files = vec![];
    let mut visit = vec![path];
    while let Some(path) = visit.pop() {
        for dir in read_dir(path)? {
            let entry = dir?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                files.push(entry.path());
            } else {
                visit.push(entry.path())
            }
        }
    }

    Ok(files)
}

pub fn humanize_bytes(bytes: usize) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut byte_size = bytes as f64;
    let mut index = 0;
    while byte_size >= 1024.0 && index < sizes.len() - 1 {
        byte_size /= 1024.0;
        index += 1;
    }
    format!("{:.2} {}", byte_size, sizes[index])
}
