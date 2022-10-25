/// Clap documentation: https://docs.rs/clap/latest/clap/
/// How to test clap: https://www.fpcomplete.com/rust/command-line-parsing-clap/
/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod accept;
mod cli;

use cli::Args;
use walkdir::WalkDir;

fn main() {
    let args = Args::new();
    let walker = setup_walker(&args);

    for dir_entry in walker
        .into_iter()
        .filter_entry(|item| accept::accept_path(&args, item))
    {
        // Get next entry
        let item = match dir_entry {
            Err(err) => {
                if args.debug {
                    eprintln!("ERR: {:?}", err);
                }
                continue;
            }
            Ok(item) => item,
        };

        // TODO: move into a function
        let path = item.path();
        let path = path.to_owned();
        let path = path.as_path();
        let path = path.display().to_string();
        let path = path.replace("\\", "/");
        let (_, last) = path.split_at(args.start_path.as_os_str().len()); // +1 if start_path does not end with / or \

        // looks like this resolves link traversal
        //let path = fs::canonicalize(item.path()).unwrap();

        println!("{}", last);
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
