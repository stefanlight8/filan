use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "analyze", about = "Analyze file types in a directory")]
pub struct Args {
    /// Target directory
    pub dir: Option<PathBuf>,

    /// Sort method
    #[arg(short, long, default_value = "size", value_enum)]
    pub sort: SortField,

    /// Limit how many extensions need to be shown
    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum SortField {
    Size,
    Name,
    Count,
}
