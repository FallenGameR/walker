/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod accept;
mod args;
mod utils;

use args::Args;
use utils::normalize;
use walkdir::{WalkDir, DirEntry};

fn main() {
    let args = Args::new();
    let walker = setup_walker(&args);

    // Walk the dirs from the start path
    for dir_entry in walker
        .into_iter()
        .filter_entry(|item| accept::accept_path(&args, item))
    {
        let path = match dir_entry {
            Err(error) => {
                if args.verbose {
                    eprintln!("ERR: {:?}", error);
                }
                continue;
            }
            Ok(entry) => trim(&args, &entry),
        };

        println!("{}", path);
    }
}

fn setup_walker(args: &Args) -> WalkDir {
    let mut walker = WalkDir::new(&args.start_dir);
    walker = walker.contents_first(args.breadth_first_search);
    walker = walker.follow_links(args.traverse_links);
    // this is broken for root paths like d:\
    // for these paths we need to add min depth of 1 in order to start enumeration
    // and explicitly add d:\ as the output folder
    walker = walker.min_depth(0);
    walker = walker.max_depth(args.depth.unwrap_or(usize::MAX));
    walker
}

// Walk dir has problems enumerating paths from the root drives =(

fn trim(args: &Args, item: &DirEntry) -> String {
    // add . before / - otherwise it looks like a absolute path in unix
    let path = normalize(item.path().display());

    if args.absolute_paths {
        return path
    }

    let path = "./".to_owned() + path.split_at(args.start_prefix_len).1;
    path
}
