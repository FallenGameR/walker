use std::path::PathBuf;
use clap::Parser;

/// * Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// * Add files to the output (cdf / codef)
    #[arg(short = 'f', long, value_name = "true|false")]
    pub add_files: bool,

    /// ? Add the current folder to the output
    #[arg(short = 'c', long)]
    pub add_current_folder: bool,

    /// * Add entries that start with dot to the output (hidden on unix systems)
    #[arg(short = '.', long, value_name = "dots")]
    pub add_dots: bool,

    /// * Add entries with hidden NTFS attribute to the output (hidden on windows systems)
    #[arg(short = 'n', long, value_name = "hidden")]
    pub add_hidden: bool,

    /// List the most deep entries first
    #[arg(short = 'l', long, value_name = "leaf")]
    pub leafs_first: bool,

    /// Traverse symbolic links
    #[arg(short = 't', long, value_name = "link")]
    pub link_traversal: bool,

    /// Maximum depth of traversal, unlimited by default
    #[arg(short, long)]
    pub depth: Option<usize>,

    /// Path to start from (current folder by default)
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// * Regular expression that file names need to match
    #[arg(short, long)]
    pub regex: Option<String>,

    /// * List of injected entries (favorites)
    #[arg(short, long)]
    pub injected: Vec<PathBuf>,

    /// * List of excluded entry names
    #[arg(short, long)]
    pub excluded: Vec<PathBuf>,
}
