use std::env::{args, current_dir};
use std::io::Error;
use std::path::PathBuf;

use analyze::analyze;

mod analyze;
mod utils;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    let dir_path: PathBuf = if let Some(path) = args.get(1) {
        path.parse().expect("Failed to parse")
    } else {
        current_dir()?
    };
    println!("Directory: {}", dir_path.display());
    return analyze(dir_path);
}
