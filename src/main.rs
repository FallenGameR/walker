/// Clap documentation: https://docs.rs/clap/latest/clap/
/// How to test clap: https://www.fpcomplete.com/rust/command-line-parsing-clap/
/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod cli;

use std::{path::{Path, PathBuf}, fs::DirEntry};
use clap::Parser;
use walkdir::WalkDir;
use cli::Cli;
use std::fs;

/// erer
fn main() {
    let args = Cli::parse();



    walk(&args);
}

fn walk(args: &Cli){
    // into_iter does recursive walk
    // traverses alphabetically even when .contents_first(true) is called

    let regular = "C:/Users/alexko/Downloads";
    let oneDrive = "d:/OneDrive/Projects/Coding/CoreXtAutomation/";
    let oneDriveCyrilic = "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/";
    let default = PathBuf::from(regular);

    let path = args.path.as_ref().unwrap_or(&default);

    let mut walker = WalkDir::new(path);
    walker = walker.contents_first(args.leafs_first);
    walker = walker.follow_links(args.link_traversal);

    if args.add_current_folder {
        walker = walker.min_depth(0);
    }
    else {
        walker = walker.min_depth(1);
    }

    if let Some(depth) = args.depth {
        walker = walker.max_depth(depth);
    }

    for file in walker.into_iter().filter_map(|dir_entry| dir_entry.ok())
    {
        if !args.add_files && file.metadata().unwrap().is_file() {
            continue;
        }

        //canonicalize(&self)
        // / separators
        // skip common prefix

        let path = fs::canonicalize(file.path()).unwrap();
        println!("{:?}", path.as_path().display().to_string().replace("\\","/"));
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
