use std::fs::{read_dir, read_link};
use std::io::Error;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;

pub fn walk_dir(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut files = vec![];
    let mut visit = vec![path];
    while let Some(mut path) = visit.pop() {
        if path.is_symlink() {
            path = read_link(path)?;
            if !path.exists() || !path.metadata()?.file_type().is_block_device() {
                continue;
            }
        };
        for dir in read_dir(path)? {
            match dir {
                Ok(entry) => {
                    let file_type = entry.file_type()?;
                    if file_type.is_file() {
                        files.push(entry.path());
                    } else if file_type.is_dir() || file_type.is_symlink() {
                        visit.push(entry.path())
                    }
                }
                Err(_) => continue,
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
