/// Clap documentation: https://docs.rs/clap/latest/clap/
/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html

use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

/// Fast folder walker to be used as replacement for the default fzf walker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Add files to the output (cdf / codef)
    #[arg(short = 'f', long, value_name = "true|false")]
    add_files: bool,

    /// Add the current folder to the output
    #[arg(short = 'c', long)]
    add_current_folder: bool,

    /// Add entries that start with dot to the output (hidden on unix systems)
    #[arg(short = '.', long, value_name = "dots")]
    add_dots: bool,

    /// Add entries with hidden NTFS attribute to the output (hidden on windows systems)
    #[arg(short = 'n', long, value_name = "hidden")]
    add_hidden: bool,

    /// List the most deep entries first
    #[arg(short = 'l', long, value_name = "leaf")]
    leafs_first: bool,

    /// Traverse symbolic links
    #[arg(short = 't', long, value_name = "link")]
    link_traversal: bool,

    /// Maximum depth of traversal, unlimited by default
    #[arg(short, long)]
    depth: usize,

    /// List of injected entries (favorites)
    #[arg(short, long)]
    injected: Vec<PathBuf>,

    /// List of excluded entry names
    #[arg(short, long)]
    excluded: Vec<PathBuf>,
}

/// erer
fn main() {
    let args = Cli::parse();

}

fn walk_folders(){
    // into_iter does recursive walk
    // traverses alphabetically even when .contents_first(true) is called

    let regular = "C:/Users/alexko/Downloads";
    let oneDrive = "d:/OneDrive/Projects/Coding/CoreXtAutomation/";
    let oneDriveCyrilic = "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/";
    let path = regular;

    for file in WalkDir::new(oneDriveCyrilic).contents_first(true).min_depth(1)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        /*
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
        }
        */

        //canonicalize(&self)
        // / separators
        // skip common prefix

        println!("{:?}", file.path().display());
    }


    // iter.follow_links(true) - start following symbolic links
}

fn walk_hidden_folders() {
    /*
    use walkdir::{DirEntry, WalkDir};

    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
    }

let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
*/
}
