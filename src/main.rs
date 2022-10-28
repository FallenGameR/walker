/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod accept;
mod args;
mod utils;

use std::{path::PathBuf, fs, os::windows::prelude::MetadataExt};
use args::Args;
use utils::normalize;
use walkdir::{WalkDir, DirEntry};
use anyhow::Result;

#[derive(Debug)]
pub struct Node {
    /// Path reported by fs::ReadDir, see https://doc.rust-lang.org/stable/std/fs/struct.ReadDir.html
    path: PathBuf,

    /// The depth of the node relative to the root that has depth of 0
    depth: usize,

    /// File metadata that we get cheaply in Windows and it is the correct way to handle
    /// hidden attributes and OneDrive folders since they are reported as reparse points.
    /// Standart rust library is not very helpful here, see https://github.com/rust-lang/rust/issues/46484
    metadata: fs::Metadata,
}

/*
https://doc.rust-lang.org/std/fs/#
https://doc.rust-lang.org/std/fs/fn.read_dir.html
https://doc.rust-lang.org/std/fs/struct.DirEntry.html - metadata is cheap to call, reads from buffer that is populated with lots of entries in the same folder
https://docs.rs/jwalk/latest/jwalk/ - test if parrallelizm is a thing
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html is_symlink_dir
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.MetadataExt.html file_attributes
*/

fn main() -> Result<()> {
    let args = Args::new();

    let depth = 0;

    let path = PathBuf::from(&args.start_dir);
    let meta = fs::metadata(&path)?;
    let root = Node{ path: path, depth: 0, metadata: meta };


    let iterator = match fs::read_dir(&args.start_dir) {
        Ok(iterator) => iterator,
        Err(error) => {
            if args.verbose {
                eprintln!("ERR: failed to read directory {}, error {:?}", &args.start_dir, error);
            }
            return Ok(());
        }
    };

    for entry in iterator {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                if args.verbose {
                    eprintln!("ERR: failed to open file system entry, error {:?}", error);
                }
                continue;
            }
        };

        let meta = match entry.metadata()
        {
            Ok(meta) => meta,
            Err(error) => {
                if args.verbose {
                    eprintln!("ERR: failed to read medata for file system entry, error {:?}", error);
                }
                continue;
            }
        };

        let node = Node{ path: entry.path(), depth: depth + 1, metadata: meta };

        println!("{:?}", node);
    }

    Ok(())

    /*
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

    */
}

fn trim(args: &Args, item: &DirEntry) -> String {
    // add . before / - otherwise it looks like a absolute path in unix
    let path = normalize(item.path().display());

    if args.absolute_paths {
        return path
    }

    let path = "./".to_owned() + path.split_at(args.start_dir.len()).1;
    path
}
