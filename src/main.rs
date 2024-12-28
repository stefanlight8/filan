use std::env::current_dir;
use std::io::Error;

use analyze::analyze;

mod analyze;
mod structs;
mod utils;

fn main() -> Result<(), Error> {
    let dir_path = current_dir()?;
    println!("Directory: {}", dir_path.display());
    return analyze(dir_path);
}
