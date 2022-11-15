mod args;
mod node;
use args::Args;
use crossbeam_channel::unbounded;
use node::Node;
use threadpool::ThreadPool;
use std::{fs, path::PathBuf, thread};

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
            let root = Node::new_root(path, meta);
            walk(&args, root);
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
///
/// Adapt https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html#calculate-sha256-sum-of-iso-files-concurrently
/// But us mpmc channel
///
/// Convert from e?printl to enum - Actions ShowPath/Skip/Exclude/Error or something like that
///
/// https://docs.rs/crossbeam/0.8.2/crossbeam/macro.select.html
///
fn walk(args: &Args, root: Node) {

    // Prepare thread pool
    let (s, r) = unbounded();
    let logical_cpus = args.threads.unwrap();
    let pool = ThreadPool::new(logical_cpus);

    for _ in 0..logical_cpus {
        let args = args.to_owned();
        let (s, r) = (s.clone(), r.clone());


        let lambda = move || {

            let thread = thread::current();
            let id = thread.id();
            print!("{id:?}/");

            while !r.is_empty() { // rather until there is no work for either thread
                let node: Node = match r.recv() {
                    Ok(node) => node,
                    Err(err) => {
                        if args.verbose {
                            println!("Problem receiving from unbound channel, error {err:?}");
                        }
                        continue;
                    }
                };

                /*
                // Exclude the entry and its descendants
                if is_excluded(&args, &node) {
                    continue;
                }

                // Show path if allowed
                let path = trim(&args, &node);

                if needs_showing(&args, &node, &path) {
                    let id = thread::current().id();
                    print!("{id:?} - ");
                    show(&args, &node, &path);
                }

                // Traverse only directories
                if !node.is_directory() {
                    continue;
                }

                // Traverse symlinks by default, but this could have been disabled
                if node.is_link() && args.dont_traverse_links {
                    if args.verbose {
                        println!(
                            "Skipping traversal of {} symlink folder because arguments say so | {node:?}",
                            node.path.display()
                        );
                    }
                    continue;
                }
                */
                // Prepare the traversal
                let iterator = match fs::read_dir(&node.path) {
                    Ok(iterator) => iterator,
                    Err(error) => {
                        if args.verbose {
                            eprintln!(
                                "ERR: failed to read directory {error}, error {:?}",
                                &node.path.display()
                            );
                        }
                        continue;
                    }
                };

                // Convert to nodes and do the traversal
                for entry in iterator {
                    if let Some(new_node) = Node::new(&args, entry, &node.depth + 1) {
                        match s.send(new_node) {
                            Ok(()) => (),
                            Err(err) => {
                                if args.verbose {
                                    println!("Problem sending new_node to unbound channel, error {err:?}");
                                }
                                continue;
                            },
                        };
                    }
                }
            }
        };

        // Start new thread
        pool.execute(lambda);
    }

    // Prepare the iteration
    match s.send(root) {
        Ok(()) => (),
        Err(err) => {
            if args.verbose {
                println!("Problem sending root to unbound channel, error {err:?}");
            }
            return;
        },
    };

    pool.join();
}

fn is_excluded(args: &Args, node: &Node) -> bool {
    // Exclude unresolvable
    let file_entry_name = match node.path.file_name() {
        Some(name) => name,
        None => {
            if args.verbose {
                eprintln!("ERR: failed to get the file name for the node | {node:?}");
            }
            return true;
        }
    };

    // Exclude when max depth limit is reached
    if node.depth > args.max_depth_resolved {
        if args.verbose {
            println!(
                "Excluding {} entry because max depth limit of {} is reached | {node:?}",
                file_entry_name.to_string_lossy(),
                args.max_depth_resolved
            );
        }
        return true;
    }

    // Exclude explicitly excluded
    for excluded in &args.excluded {
        let lowercase = file_entry_name.to_ascii_lowercase();
        let lowercase = lowercase.to_string_lossy();
        if lowercase == excluded.as_str() {
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

    // Exclude dots (done by default)
    if !args.show_dots && node.is_dot() {
        if args.verbose {
            println!(
                "Excluding {} entry because arguments say to exclude dots | {node:?}",
                file_entry_name.to_string_lossy()
            );
        }
        return true;
    }

    // Exclude hidden (done by default)
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

pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}

fn needs_showing(args: &Args, node: &Node, path: &str) -> bool {
    // Hide files
    if args.hide_files && node.is_file() {
        if args.verbose {
            println!("Hiding {path} file because arguments say to hide files | {node:?}");
        }
        return false;
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
