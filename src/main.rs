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
            Ok(entry) => normalize2(&args, &entry),
        };

        println!("{}", path);
    }
}

fn setup_walker(args: &Args) -> WalkDir {
    let mut walker = WalkDir::new(&args.start_path);
    walker = walker.contents_first(args.leafs_first);
    walker = walker.follow_links(args.link_traversal);
    walker = walker.min_depth(if args.add_current_folder { 0 } else { 1 });
    walker = walker.max_depth(args.depth.unwrap_or(usize::MAX));
    walker
}

fn normalize2(args: &Args, item: &DirEntry) -> String {
    // looks like this resolves link traversal
    // let path = fs::canonicalize(item.path()).unwrap();

    let path = normalize(item.path().display());
    let (_, path) = path.split_at(args.start_path_trim);
    String::from( if path.len() <= 1 {"."} else {path} )
}
