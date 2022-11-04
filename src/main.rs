mod args;
mod node;
use args::Args;
use node::Node;
use std::{fs, path::PathBuf};
//use anyhow::Error;
//use walkdir;
//use anyhow::Result;

/*
/// Environment variables: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
https://docs.rs/jwalk/latest/jwalk/ - test if parrallelizm is a thing
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html is_symlink_dir
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.MetadataExt.html file_attributes
*/

fn main() {
    let args = Args::new();

    // Arguments sanity check
    if args.hide_files && args.hide_directories && !args.show_root && (args.included.len() == 0) {
        eprintln!("ERR: nothing to show, arguments instruct to hide files, directories, root and nothing is injected");
        return;
    }

    // Injections are inserted here
    for node in args
        .included
        .iter()
        .filter_map(|path| Node::new_injected(&args, &path))
    {
        // Don't trim and ignore -fd flags
        let path = node.path.display().to_string();
        show(&args, &node, &path);
    }

    // Insert start directory here
    if args.show_root {
        if let Some(node) = Node::new_injected(&args, &args.start_dir) {
            // Trim but ignore -fd flags
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

/// To support breadth first approach and parrallelizm we need
/// not to use recursion here but rather use queue/stack
fn walk(args: &Args, root: &Node) {
    // Limit traversal
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
        // Create node to process and walk through
        let node = match Node::new(&args, entry, &root.depth + 1) {
            Some(node) => node,
            None => continue,
        };

        // Skip the entry and its descendants
        if exclude(&args, &node) {
            continue;
        }

        render(&args, &node);

        // Traverse children
        if node.metadata.is_dir() {
            walk(&args, &node);
        }
    }
}

fn exclude(args: &Args, node: &Node) -> bool {
    let file_entry_name = match node.path.file_name() {
        Some(name) => name,
        None => return false, // Fact there is no file name doesn't mean we can't walk it, does it?
    };

    // Exclude excluded
    for excluded in &args.excluded {
        if file_entry_name == excluded.as_str() {
            if args.verbose {
                println!(
                    "Excluding {} entry because arguments say to exclude {} | {node:?}",
                    file_entry_name.to_string_lossy(),
                    excluded
                );
            }
            return true;
        }
    }

    // Exclude dots (by default)
    if !args.show_dots && node.is_dot() {
        if args.verbose {
            println!(
                "Excluding {} entry because arguments say to exclude dots | {node:?}",
                file_entry_name.to_string_lossy()
            );
        }
        return true;
    }

    // Exclude hidden (by default)
    if !args.show_hidden && node.is_hidden() {
        if args.verbose {
            println!(
                "Excluding {} entry because arguments say to exclude hidden | {node:?}",
                file_entry_name.to_string_lossy()
            );
        }
        return true;
    }

    false
}

/// Inline?
fn render(args: &Args, node: &Node) {
    // The path to output
    let path = trim(&args, &node);

    // Show if accepted if needed
    if show_entry(args, node, &path) {
        show(&args, &node, &path);
    }
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

/// Inline?
pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}

pub fn show_entry(args: &Args, node: &Node, path: &str) -> bool {
    // Hide files
    if args.hide_files && node.is_file() {
        if args.verbose {
            println!("Hiding {path} file because arguments say to hide files | {node:?}");
        }
        return true;
    }

    // Hide directories, but it is still walked
    if args.hide_directories && node.is_directory() {
        if args.verbose {
            println!(
                "Hiding {path} directory because arguments say to hide directories | {node:?}"
            );
        }
        return false;
    }

    true
}

fn show(args: &Args, node: &Node, path: &str) {
    if args.verbose {
        println!("{path} | {node:?}");
    } else {
        println!("{path}");
    }
}
