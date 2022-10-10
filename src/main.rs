/*
https://docs.rs/clap/latest/clap/
https://github.com/clap-rs/clap/tree/45241277043f2a8cc64230e18574b88b005e765c/examples
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
*/

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional list of excluded folders names (taken from file)
    excluded: Option<Vec<PathBuf>>,

    /// Flag that says if files are needed in the output (cdf / codef)
    add_files: bool,

    /// Flag that says if the current folder is needed in the output
    add_current_folder: bool,

    /// Optional list of injected folders (favorites)
    injected: Option<Vec<PathBuf>>,

    /// Optional maximum depth of traversal, unlimited by default
    #[arg(short, long)]
    depth: Option<usize>,

    /// Flag that says if the most deep entries need to be displayed first
    #[arg(short, long, value_name = "leaf")]
    leafs_first: bool,

    /// Flag that says if the symbolic links need to be traversed
    #[arg(short, long, value_name = "link")]
    link_traversal: bool,

    /// Flag that says if the entries that start with dot need to be skipped (hidden on unix systems)
    #[arg(short, long, value_name = ".")]
    skip_dots: bool,

    /// Flag that says if the entries that have hidden NTFS attribute need to be skipped (hidden on windows systems)
    #[arg(short, long, value_name = "h")]
    skip_hidden: bool,
}

fn main() {
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
