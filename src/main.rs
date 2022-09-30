use clap::{Parser, Subcommand};
use walkdir::WalkDir;

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
        // no prefix
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

/*
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}
*/


/*
Requirements
------------
- excluded folders: optional list of names (taken from file?)
- do list files: flag (cdf / codef)
- be faster than similar PS code - test on huge folder
- skip common prefix
- inject quick lists: optiona list of paths
- max depth: optional number
- traversal order: flag
- add current folder: flag
- link traversal: flag
- skip entries that start with dot: flag
- skip hidden entries: flag


https://docs.rs/clap/latest/clap/
https://github.com/clap-rs/clap/tree/45241277043f2a8cc64230e18574b88b005e765c/examples
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
*/