use std::env::current_dir;
use std::io::Result;

use analyze::get_files_types;
use utils::humanize_bytes;

mod analyze;
mod structs;
mod utils;

fn main() -> Result<()> {
    let dir_path = current_dir()?;
    println!("Directory: {}", dir_path.display());
    for (file_type, data) in get_files_types(dir_path).lock().unwrap().iter() {
        println!("{}", format!("{}: {} files, {} size", file_type, data.count, humanize_bytes(data.size)))
    }
    Ok(())
}
