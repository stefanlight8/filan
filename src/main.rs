use std::{env::current_dir, io::Error, path::PathBuf};

use analyze::analyze;
use clap::Parser;

use crate::interface::Args;

mod analyze;
mod interface;
mod utils;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let target_dir: PathBuf = args.dir.unwrap_or(current_dir().unwrap());
    println!("Directory: {}", &target_dir.display());
    return analyze(target_dir, args.sort, args.limit);
}
