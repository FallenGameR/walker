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

fn walk(args: &Args, root: &Node) {

    let iterator = match fs::read_dir(&root.path) {
        Ok(iterator) => iterator,
        Err(error) => {
            if args.verbose {
                eprintln!("ERR: failed to read directory {}, error {:?}", &root.path.display(), error);
            }
            return;
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
                    eprintln!("ERR: failed to read metadata for file system entry, error {:?}", error);
                }
                continue;
            }
        };

        let node = Node{ path: entry.path(), depth: root.depth + 1, metadata: meta };

        if args.verbose {
            println!("{:?}", node);
        }
        else{
            let path = trim(&args, &node);
            println!("{}", path);
        }

        // Make sure it works for OneDrive folders
        if node.metadata.is_dir() {
            walk(args, &node);
        }
    }
}

fn main(){
    let args = Args::new();
    let path = PathBuf::from(&args.start_dir);

    match fs::metadata(&path)
    {
        Ok(meta) => {
            let root = Node{ path: path, depth: 0, metadata: meta };
            walk(&args, &root);
        },
        Err(error) => {
            if args.verbose {
                eprintln!("ERR: failed to read metadata for start path {:?}, error {:?}", &path, error);
            }
        }
    };
}

fn trim(args: &Args, item: &Node) -> String {
    // add . before / - otherwise it looks like a absolute path in unix
    let path = normalize(item.path.display());

    if args.absolute_paths {
        return path
    }

    let path = ".\\".to_owned() + path.split_at(args.start_dir.len()).1;
    path
}

// tests
// OneDriveFolder
// Drive root
// regular folder