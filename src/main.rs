/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
mod args;
use std::{
    fs::{self, DirEntry},
    os::windows::prelude::MetadataExt,
    path::PathBuf,
};
//use anyhow::Error;
use args::Args;
//use walkdir;
use winapi::um::winnt;
//use anyhow::Result;

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

impl Node {
    fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    /// Returns true if and only if this entry points to a directory.
    ///
    /// This works around a bug in Rust's standard library:
    /// https://github.com/rust-lang/rust/issues/46484
    fn is_directory(&self) -> bool {
        self.metadata.file_attributes() & winnt::FILE_ATTRIBUTE_DIRECTORY != 0
        //self.metadata.is_dir()
    }

    fn new(args: &Args, entry: Result<DirEntry, std::io::Error>, depth: usize) -> Option<Node> {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                if args.verbose {
                    eprintln!("ERR: failed to open file system entry, error {:?}", error);
                }
                return None;
            }
        };

        let meta = match entry.metadata() {
            Ok(meta) => meta,
            Err(error) => {
                if args.verbose {
                    eprintln!(
                        "ERR: failed to read metadata for file system entry, error {:?}",
                        error
                    );
                }
                return None;
            }
        };

        Some(Node {
            path: entry.path(),
            depth: depth,
            metadata: meta,
        })
    }

    fn new_injected(args: &Args, path: &str) -> Option<Node> {
        let path = PathBuf::from(path);
        if !path.exists() {
            if args.verbose {
                eprintln!(
                    "ERR: skipping injected path {} since it does not exist",
                    path.display()
                );
            }
            return None;
        }

        let meta = match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(error) => {
                if args.verbose {
                    eprintln!(
                        "ERR: failed to read metadata for injected path {}, error {:?}",
                        path.display(),
                        error
                    );
                }
                return None;
            }
        };

        Some(Node {
            path: path,
            depth: 0,
            metadata: meta,
        })
    }
}
/*
https://doc.rust-lang.org/std/fs/#
https://doc.rust-lang.org/std/fs/fn.read_dir.html
https://doc.rust-lang.org/std/fs/struct.DirEntry.html - metadata is cheap to call, reads from buffer that is populated with lots of entries in the same folder
https://docs.rs/jwalk/latest/jwalk/ - test if parrallelizm is a thing
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html is_symlink_dir
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.MetadataExt.html file_attributes
*/


fn show(args: &Args, node: &Node, path: &str) {
    if args.verbose {
        println!("{path} | {node:?}");
    } else {
        println!("{path}");
    }
}

fn main() {
    let args = Args::new();

    // Arguments sanity check
    if args.hide_files && args.hide_directories && (args.injected.len() == 0) {
        eprintln!("ERR: nothing to show, arguments instruct to hide both files and directories and nothing is injected");
        return;
    }

    // Injections are inserted here
    for path in &args.injected {
        let node = match Node::new_injected(&args, &path) {
            Some(node) => node,
            None => continue,
        };

        let path = node.path.display().to_string();
        show(&args, &node, &path);
    }

    // Inject root folder here
    if args.show_root {
        if let Some(node) = Node::new_injected(&args, &args.start_dir) {
            let path = trim(&args, &node);
            show(&args, &node, &path);
        };
    }

    // Start walking from the start directory
    let path = PathBuf::from(&args.start_dir);

    match fs::metadata(&path) {
        Ok(meta) => {
            let root = Node {
                path: path,
                depth: 0,
                metadata: meta,
            };
            walk(&args, &root);
        }
        Err(error) => {
            if args.verbose {
                eprintln!(
                    "ERR: failed to read metadata for start path {:?}, error {:?}",
                    &path, error
                );
            }
        }
    };
}

fn walk(args: &Args, root: &Node) {
    if root.depth >= args.max_depth.unwrap_or(usize::MAX) {
        return;
    }

    let iterator = match fs::read_dir(&root.path) {
        Ok(iterator) => iterator,
        Err(error) => {
            if args.verbose {
                eprintln!(
                    "ERR: failed to read directory {}, error {:?}",
                    &root.path.display(),
                    error
                );
            }
            return;
        }
    };

    for entry in iterator {
        let node = match Node::new(&args, entry, &root.depth + 1) {
            Some(node) => node,
            None => continue,
        };

        if exclude(&args, &node) {
            continue;
        }

        render(&args, &node);

        if node.metadata.is_dir() {
            walk(&args, &node);
        }
    }
}

fn exclude(args: &Args, node: &Node) -> bool {
    let file_entry_name = match node.path.file_name() {
        Some(name) => name,
        None => return false,
    };

    for excluded in &args.excluded {
        if file_entry_name == excluded.as_str() {
            return true;
        }
    }

    false
}

fn render(args: &Args, node: &Node) {
    // The path to output
    let path = trim(&args, &node);

    // Hide files
    if args.hide_files && node.is_file() {
        if args.verbose {
            println!("Hiding {path} file because arguments say to hide files | {node:?}");
        }
        return;
    }

    // Hide folders
    if args.hide_directories && node.is_directory() {
        if args.verbose {
            println!(
                "Hiding {path} directory because arguments say to hide directories | {node:?}"
            );
        }
        return;
    }

    // Show node
    show(&args, &node, &path);
}

fn trim(args: &Args, item: &Node) -> String {
    let path = normalize(item.path.display());

    if args.absolute_paths {
        return path;
    }

    // use .\ prefix, otherwise it will look like /usr - absolute path in unix
    let removed = args.start_dir.len();
    let remaining = path.len() - removed;
    let mut result = String::with_capacity(2 + remaining);
    result.push('.');
    result.push(std::path::MAIN_SEPARATOR);
    result.push_str(path.split_at(removed).1);
    result
}

// tests
// OneDriveFolder
// Drive root
// regular folder

pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}

pub fn accept_path(args: &Args, node: &Node) -> bool {
    if !args.hide_files && is_file(node) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_file", node.path.display());
        }
        return false;
    }

    if !args.show_dots && is_dot(node) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_dot", node.path.display());
        }
        return false;
    }

    if !args.show_hidden && is_hidden(node) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_hidden", node.path.display());
        }
        return false;
    }

    if args.verbose {
        eprintln!("dbg> {} - accepted", node.path.display());
    }
    true
}

fn is_file(node: &Node) -> bool {
    node.metadata.is_file()
}

fn is_dot(node: &Node) -> bool {
    node.path
        .file_name()
        .map_or(false, |s| s.to_string_lossy().starts_with("."))
}

fn is_hidden(node: &Node) -> bool {
    //println!("TEST {}", item.path().display());
    let meta = &node.metadata;

    //println!("META {}", meta.file_attributes());
    let attributes = meta.file_attributes();
    let hidden = (attributes & 0x2) != 0;
    let system = (attributes & 0x4) != 0;
    let directory = (attributes & 0x16) != 0;

    if hidden {
        // Drive roots have hidden flag set but we need to list them regardless.
        if hidden && system && directory && node.path.parent().is_none() {
            return false;
        }
        return true;
    }

    //println!("META no");
    false
}
