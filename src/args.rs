use clap::Parser;
use std::path::PathBuf;

use crate::utils::normalize;

/// Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Start path from where to walk
    #[clap(skip)]
    pub start_dir: PathBuf,

    /// How many characters to trim from the result paths
    #[clap(skip)]
    pub start_prefix_len: usize,

    /// Add files to the output (cdf / codef)
    #[arg(short = 'f', long, value_name = "true|false")]
    pub add_files: bool,

    /// ? Add the current folder to the output
    #[arg(short = 'c', long)]
    pub add_current_folder: bool,

    /// ** Add entries that start with dot to the output (hidden on unix systems)
    #[arg(short = '.', long, value_name = "dots")]
    pub add_dots: bool,

    /// ** Add entries with hidden NTFS attribute to the output (hidden on windows systems)
    #[arg(short = 'w', long, value_name = "hidden")]
    pub add_hidden: bool,

    /// ** List the most deep entries first
    #[arg(short = 'l', long, value_name = "leaf")]
    pub leafs_first: bool,

    /// ** Traverse symbolic links
    #[arg(short = 't', long, value_name = "link")]
    pub link_traversal: bool,

    /// Use absolute paths, don't trim the output
    #[arg(short, long)]
    pub absolute: bool,

    /// ** Verbose output for debugging
    #[arg(short, long)]
    pub verbose: bool,

    /// ** Turn debugging information on
    #[arg(long)]
    pub debug: bool,

    /// ** Maximum depth of traversal, unlimited by default
    #[arg(short, long)]
    pub depth: Option<usize>,

    /// ** Path to start from (current folder by default)
    #[arg(short, long)]
    pub path: Option<String>,

    /// ** Regular expression that file names need to match
    #[arg(short, long)]
    pub regex: Option<String>,

    /// ** List of injected entries (favorites)
    #[arg(short, long)]
    pub injected: Vec<String>,

    /// ** List of excluded entry names
    #[arg(short, long)]
    pub excluded: Vec<String>,
}

impl Args {
    // Resolve start path and make sure it is valid:
    // - is a folder
    // - needs to use correct /\
    // - needs not to have trailing /
    fn resolve_start_dir(path: &Option<String>) -> PathBuf {
        // Resolve initial value
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => std::env::current_dir().unwrap(), // uses \ no trailing \
        };

        // Make sure it is a folder
        match path.metadata() {
            Ok(meta) => if !meta.is_dir() {
                panic!("ERR: path {} needs be a dirrectory but it was not", path.display());
            },
            Err(error) => panic!(
                "ERR: could not get file or directory \
                metadata for {}, error is {}",
                path.display(),
                error),
        };

        // Make sure correct slashes are used
        let mut path = path.display().to_string().replace("/", "\\");

        // Make sure trailing slash is present
        if !path.ends_with("\\") {
            path.push('\\');
            //let mut chars = path.chars();
            //chars.next_back();
            //path = chars.as_str().to_string();
        }

        // Make it an owned path
        PathBuf::from(path)
    }

    pub fn new() -> Args {
        let mut args = Args::parse();
        args.start_dir = Self::resolve_start_dir(&args.path);
        args.start_prefix_len = normalize(args.start_dir.display()).len() - 1;
        args
    }
}