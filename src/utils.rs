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
