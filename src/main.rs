/// Clap documentation: https://docs.rs/clap/latest/clap/
/// How to test clap: https://www.fpcomplete.com/rust/command-line-parsing-clap/
/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod cli;

use clap::Parser;
use cli::Cli;
use std::{fs, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args = Cli::parse();

    walk(&args);
}

fn walk(args: &Cli) {
    let current_folder = std::env::current_dir().unwrap();
    let current_folder = PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(&current_folder);
    let mut walker = WalkDir::new(path);

    walker = walker.contents_first(args.leafs_first);
    walker = walker.follow_links(args.link_traversal);
    walker = walker.min_depth(if args.add_current_folder { 0 } else { 1 });
    walker = walker.max_depth(args.depth.unwrap_or(usize::MAX));

    fn is_dot(item: &DirEntry) -> bool {
        item.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    let test_traversal = |item: &DirEntry| -> bool {
        if !args.add_dots && is_dot(item) {
            return false;
        }
        true
    };

    // NOTE: it is fine to redefine variable type as long as there is another let
    //let dirs = fs::read_dir(".foo").unwrap();
    //let dirs = dirs.map(|file| file.unwrap().path());

    // https://doc.rust-lang.org/std/option/index.html option docs
    for dir_entry in walker
        .into_iter()
        .filter_entry(|dir_entry| test_traversal(dir_entry))
    {
        let item = dir_entry.unwrap();

        if !args.add_files && item.metadata().unwrap().is_file() {
            continue;
        }

        //println!("  {:?}", item.file_name().to_str());

        if !args.add_dots && is_dot(&item) {
            continue;
        }

        // skip common prefix

        // looks like this resolves link traversal
        //let path = fs::canonicalize(item.path()).unwrap();
        let path = item.path();
        let path = path.to_owned();
        let path = path.as_path();
        let path = path.display().to_string();
        let path = path.replace("\\", "/");

        println!("{:?}", path);
    }
}
