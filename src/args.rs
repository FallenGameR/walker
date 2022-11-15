use crate::normalize;
use clap::Parser;
use std::path::PathBuf;

/// Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Start directory from where the walk started, ends with /
    #[clap(skip)]
    pub start_dir: String,

    /// Maximum depth of traversal resolved from max_depth
    #[clap(skip)]
    pub max_depth_resolved: usize,

    /// Path to start from (current folder by default)
    pub path: Option<String>,

    /// List of included entries (favorites)
    /// -a doesn't seem to affect these
    #[arg(short = 'I', long)]
    pub included: Vec<String>,

    /// List of excluded entries (just the name, it can match any part of the path)
    #[arg(short = 'e', long)]
    pub excluded: Vec<String>,

    // Include root folder to the output
    #[arg(short = 'R', long)]
    pub show_root: bool,

    /// Maximum depth of traversal, unlimited by default, children of root has depth 1
    #[arg(short = 'm', long)]
    pub max_depth: Option<usize>,

    /// Number of threads to use, not specified means 1 thread, 0 means to use all logical CPUs
    #[arg(short, long)]
    pub threads: Option<usize>,

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

impl Args {
    pub fn new() -> Args {
        let mut args = Args::parse();
        args.start_dir = Self::resolve_start_dir(&args.path);
        args.max_depth_resolved = args.max_depth.unwrap_or(usize::MAX);
        for excluded in args.excluded.iter_mut() {
            *excluded = excluded.to_lowercase();
        }
        if args.threads == None {
            args.threads = Some(1);
        }
        if args.threads == Some(0) {
            args.threads = Some(num_cpus::get());
        }
        args
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
