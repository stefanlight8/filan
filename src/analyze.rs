use std::{collections::HashMap, path::PathBuf, sync::{Arc, Mutex}};
use std::os::unix::fs::MetadataExt;

use walkdir::WalkDir;

use crate::structs::FileTypeData;

pub fn get_files_types(dir: PathBuf) -> Arc<Mutex<HashMap<String, FileTypeData>>> {
    let types: Arc<Mutex<HashMap<String, FileTypeData>>> = Arc::new(Mutex::new(HashMap::new()));
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .for_each(|file| {
            let file_path = file.path();
            if let Some(file_type) = file_path.extension().and_then(|ext| ext.to_str()) {
                let mut data = types.lock().unwrap();
                let entry = data.entry(file_type.to_string()).or_insert(FileTypeData { count: 0, size: 0 });
                entry.count += 1;
                entry.size += file.metadata().map(|m| m.size() as usize).unwrap_or(0);
            }
        });
    types
}
