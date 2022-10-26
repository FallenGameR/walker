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
                if args.debug {
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
    walker = walker.contents_first(args.leafs_first);
    walker = walker.follow_links(args.link_traversal);
    walker = walker.min_depth(if args.add_current_folder { 0 } else { 1 });
    walker = walker.max_depth(args.depth.unwrap_or(usize::MAX));
    walker
}

fn trim(args: &Args, item: &DirEntry) -> String {
    let path = normalize(item.path().display());

    if !args.absolute {
        let (_, path) = path.split_at(args.start_prefix_len);
        String::from( if path.len() <= 1 {"."} else {path} )
    }
    else {
        path
    }
}
