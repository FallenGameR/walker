use clap::Parser;
use std::path::PathBuf;

use crate::normalize;

/// Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Start directory from where the walk started, ends with /
    #[clap(skip)]
    pub start_dir: String,

    /// Path to start from (current folder by default)
    pub path: Option<String>,

    /// List of injected entries (favorites)
    #[arg(short, long)]
    pub injected: Vec<String>,

    /// List of excluded entries (just the name, it can match any part of the path)
    #[arg(short, long)]
    pub excluded: Vec<String>,

    /// ** Maximum depth of traversal, unlimited by default, starts with 1
    #[arg(short, long)]
    pub max_depth: Option<usize>,

    /// ** Traverse directory symbolic links
    #[arg(short = 'l', long)]
    pub traverse_links: bool,

    /// Exclude files from the output (cdf / codef)
    #[arg(short = 'f', long)]
    pub hide_files: bool,

    /// Exclude directories from the output (cdf / codef)
    #[arg(short = 'd', long)]
    pub hide_directories: bool,

    /// ** Add entries that start with dot (hidden on unix systems)
    #[arg(short = 'D', long)]
    pub show_dots: bool,

    /// ** Add entries with hidden NTFS attribute  (hidden on windows systems)
    #[arg(short = 'H', long)]
    pub show_hidden: bool,

    /// ** List the most deep entries first
    #[arg(long)]
    pub depth_first_search: bool,

    /// Use absolute paths, don't trim the output
    #[arg(short, long)]
    pub absolute_paths: bool,

    /// Verbose output for debugging
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    // Resolve start path and make sure it is valid:
    // - is a folder
    // - needs to use correct /\
    // - needs not to have trailing /
    fn resolve_start_dir(path: &Option<String>) -> String {
        // Resolve initial value
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => std::env::current_dir().unwrap(), // uses \ and there is no trailing \
        };

        // Make sure it is a folder
        match path.metadata() {
            Ok(meta) => if !meta.is_dir() {
                panic!("ERR: path needs be a dirrectory but it was not: {}", path.display());
            },
            Err(error) => panic!(
                "ERR: could not get metadata for {}, error is {}",
                path.display(),
                error),
        };

        // Make sure correct slashes are used
        let mut path = normalize(path.display());

        // Make sure trailing slash is present
        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            path.push(std::path::MAIN_SEPARATOR);
        }

        path
    }

    pub fn new() -> Args {
        let mut args = Args::parse();
        args.start_dir = Self::resolve_start_dir(&args.path);
        args
    }
}