use clap::Parser;
use once_cell::sync::OnceCell;
use std::{path::PathBuf, collections::HashSet, ffi::OsString};

#[derive(Debug, Clone)]
pub struct Args {
    /// Start directory from where the walk started, ends with /
    pub start_dir: String,

    /// Maximum depth of traversal resolved from max_depth
    pub max_depth: usize,

    /// Excluded folders in lowercase
    pub excluded_lowercase: HashSet<OsString>,

    /// Included folders that do exist
    pub included_existing: Vec<PathBuf>,

    /// Original command line
    pub cmd: CommandLine
}

/// Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CommandLine {
    /// Path to start from (current folder by default)
    pub path: Option<String>,

    /// List of included entries (favorites)
    /// -a doesn't seem to affect these
    #[arg(short = 'I', long)]
    pub included: Vec<String>,

    /// List of excluded entries (just the name, it can match any part of the path)
    #[arg(short = 'e', long)]
    pub excluded: Vec<String>,

    /// Include root folder to the output
    #[arg(short = 'R', long)]
    pub show_root: bool,

    /// Maximum depth of traversal, unlimited by default, children of root has depth 1
    #[arg(short = 'm', long)]
    pub max_depth: Option<usize>,

    /// Do not traverse directory symbolic links
    #[arg(short = 'l', long)]
    pub dont_traverse_links: bool,

    /// Hide files from the output (cdf / codef)
    #[arg(short = 'f', long)]
    pub hide_files: bool,

    /// Hide directories from the output, but they are still walked (cdf / codef)
    #[arg(short = 'd', long)]
    pub hide_directories: bool,

    /// Add entries that start with dot (hidden on unix systems)
    #[arg(short = 'D', long)]
    pub show_dots: bool,

    /// Add entries with hidden NTFS attribute  (hidden on windows systems)
    #[arg(short = 'H', long)]
    pub show_hidden: bool,

    /// Use absolute paths, don't trim the output
    #[arg(short, long)]
    pub absolute_paths: bool,

    /// Verbose output for debugging
    #[arg(short, long)]
    pub verbose: bool,
}

pub static ARGS: OnceCell<Args> = OnceCell::new();

impl Args {
    pub fn get() -> &'static Args {
        ARGS.get().expect("ARGS are not initialized")
    }

    pub fn cmd() -> &'static CommandLine {
        &Args::get().cmd
    }

    pub fn new() -> Args {
        let command_line = CommandLine::parse();
        Args {
            start_dir: Self::resolve_start_dir(&command_line.path),
            max_depth: command_line.max_depth.unwrap_or(usize::MAX),
            excluded_lowercase: (&command_line.excluded)
                .iter()
                .map(|e| e.to_ascii_lowercase())
                .map(|e| OsString::from(e))
                .collect(),
            included_existing: (&command_line.included)
                .iter()
                .map(|path| PathBuf::from(path))
                .filter(|path| path.exists())
                .collect(),
            cmd: command_line,
        }
    }

    /// Resolve start path and make sure it is valid:
    /// - is a folder
    /// - needs to use correct /\
    /// - needs not to have trailing /
    fn resolve_start_dir(path: &Option<String>) -> String {
        // Resolve home folder and initial value
        let path = Args::resolve_initial_value(path);

        // Make sure it is a folder
        match path.metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    panic!(
                        "ERR: path needs be a dirrectory but it was not: {}",
                        path.display()
                    );
                }
            }
            Err(error) => panic!(
                "ERR: could not get metadata for {}, error is {}",
                path.display(),
                error
            ),
        };

        // Make sure correct slashes are used
        let mut path = normalize(path.display());

        // Make sure trailing slash is present
        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            path.push(std::path::MAIN_SEPARATOR);
        }

        path
    }

    fn resolve_initial_value(path: &Option<String>) -> PathBuf {
        if let Some(path) = path {
            if path.contains("~") {
                let home = std::env::var("HOME").expect("ERR: Home environment variable needs to be defined");
                return PathBuf::from(path.replace("~", &home));
            }

            return PathBuf::from(path);
        }

        // uses \ and there is no trailing \
        std::env::current_dir().unwrap()
    }
}

fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}
