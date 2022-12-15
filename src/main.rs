mod args;
mod node;
use args::{Args, ARGS};
use crossbeam_channel::unbounded;
use jwalk::{DirEntry, Error, WalkDir, WalkDirGeneric};
use node::Node;
use std::{cmp::Ordering, fs, path::PathBuf, thread, collections::HashSet, ffi::OsString};
use threadpool::ThreadPool;

fn main() {
    ARGS.set(Args::new()).expect("Could not initialize ARGS");

    // Arguments sanity check
    if Args::get().cmd.hide_files && Args::get().cmd.hide_directories && !Args::get().cmd.show_root && (Args::get().cmd.included.len() == 0) {
        eprintln!("ERR: nothing to show, arguments instruct to hide files, directories, root and nothing is injected");
        return;
    }

    // Injections are inserted here
    for node in Args::get().cmd
        .included
        .iter()
        .filter_map(|path| Node::new_injected(&path))
    {
        // Don't trim and ignore -fd flags
        let path = node.path.display().to_string();
        show(&node, &path);
    }

    // Insert start directory here
    if Args::cmd().show_root {
        if let Some(node) = Node::new_injected(&Args::get().start_dir) {
            // Trim but ignore -fd flags
            let path = trim(&node);
            show(&node, &path);
        };
    }

    // Start walking from the start directory
    let path = PathBuf::from(&Args::get().start_dir);

    match fs::metadata(&path) {
        Ok(meta) => {
            let root = Node::new_root(path, meta);
            //walk(root); // 3s for PfGold
            jwalk(root).unwrap(); // 0.8s for PfGold
        }
        Err(error) => {
            if Args::cmd().verbose {
                eprintln!(
                    "ERR: failed to read metadata for start path {:?}, error {:?}",
                    &path, error
                );
            }
        }
    };
}

fn jwalk(root: Node) -> Result<(), Error> {
    let walk_dir = WalkDir::new(root.path);
    let excluded = &Args::get().excluded;

    let walk_dir = walk_dir.process_read_dir(|depth, parent, _, children| {
        // Don't include excluded, that makes them not traversible as well
        children.retain(|result| {
            if let Ok(entry) = result {
                let name = entry.file_name.to_ascii_lowercase();
                return !excluded.contains(&name);
            }
            false
        });

        // Don't traverse excluded, but retain them
        //children.iter_mut().for_each(|result| {
        //    if let Ok(entry) = result {
        //        let name = entry.file_name.to_ascii_lowercase();
        //        if excluded.contains(&name) {
        //            entry.read_children_path = None
        //        }
        //    }
        //});


        /*
        // 1. Custom sort
        children.sort_by(|a, b| match (a, b) {
            (Ok(a), Ok(b)) => a.file_name.cmp(&b.file_name),
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            (Err(_), Err(_)) => Ordering::Equal,
        });

        // 3. Custom skip
        children.iter_mut().for_each(|result| {
            if let Ok(entry) = result {
                if entry.depth == 2 {
                    entry.read_children_path = None;
                }
            }
        });
        // 4. Custom state
        *state += 1;
        children.first_mut().map(|result| {
            if let Ok(entry) = result {
                entry.client_state = true;
            }
        });
        */
    });

    for entry in walk_dir {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    Ok(())
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
    let logical_cpus = 1;//args.threads.unwrap();
    let pool = ThreadPool::new(logical_cpus);

    for _ in 0..logical_cpus {
        let args = args.to_owned();
        let (s, r) = (s.clone(), r.clone());

        let lambda = move || {
            // WTF: Why does printf here affects allocated threads?
            //let thread = thread::current();
            //let id = thread.id();
            //print!("{id:?}/");
            print!("");

            while !r.is_empty() {
                // rather until there is no work for either thread
                let node = match r.recv() {
                    Ok(node) => node,
                    Err(err) => {
                        if args.cmd.verbose {
                            println!("Problem receiving from unbound channel, error {err:?}");
                        }
                        continue;
                    }
                };

                // Exclude the entry and its descendants
                if is_excluded(&node) {
                    continue;
                }

                // Show path if allowed
                let path = trim(&node);

                if needs_showing(&node, &path) {
                    //let id = thread::current().id();
                    //print!("{id:?} - ");
                    show(&node, &path);
                }

                // Traverse only directories
                if !node.is_directory() {
                    continue;
                }

                // Traverse symlinks by default, but this could have been disabled
                if node.is_link() && args.cmd.dont_traverse_links {
                    if args.cmd.verbose {
                        println!(
                            "Skipping traversal of {} symlink folder because arguments say so | {node:?}",
                            node.path.display()
                        );
                    }
                    continue;
                }

                // Prepare the traversal
                let iterator = match fs::read_dir(&node.path) {
                    Ok(iterator) => iterator,
                    Err(error) => {
                        if args.cmd.verbose {
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
                    if let Some(new_node) = Node::new(entry, &node.depth + 1) {
                        match s.send(new_node) {
                            Ok(()) => (),
                            Err(err) => {
                                if args.cmd.verbose {
                                    println!("Problem sending new_node to unbound channel, error {err:?}");
                                }
                                continue;
                            }
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
            if args.cmd.verbose {
                println!("Problem sending root to unbound channel, error {err:?}");
            }
            return;
        }
    };

    pool.join();
}

fn is_excluded(node: &Node) -> bool {
    // Exclude unresolvable
    let file_entry_name = match node.path.file_name() {
        Some(name) => name,
        None => {
            if Args::cmd().verbose {
                eprintln!("ERR: failed to get the file name for the node | {node:?}");
            }
            return true;
        }
    };

    // Exclude when max depth limit is reached
    if node.depth > Args::get().max_depth_resolved {
        if Args::cmd().verbose {
            println!(
                "Excluding {} entry because max depth limit of {} is reached | {node:?}",
                file_entry_name.to_string_lossy(),
                Args::get().max_depth_resolved
            );
        }
        return true;
    }

    // Exclude explicitly excluded
    for excluded in &Args::cmd().excluded {
        let lowercase = file_entry_name.to_ascii_lowercase();
        let lowercase = lowercase.to_string_lossy();
        if lowercase == excluded.as_str() {
            if Args::cmd().verbose {
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
    if !Args::cmd().show_dots && node.is_dot() {
        if Args::cmd().verbose {
            println!(
                "Excluding {} entry because arguments say to exclude dots | {node:?}",
                file_entry_name.to_string_lossy()
            );
        }
        return true;
    }

    // Exclude hidden (done by default)
    if !Args::cmd().show_hidden && node.is_hidden() {
        if Args::cmd().verbose {
            println!(
                "Excluding {} entry because arguments say to exclude hidden | {node:?}",
                file_entry_name.to_string_lossy()
            );
        }
        return true;
    }

    false
}

fn trim(item: &Node) -> String {
    let path = normalize(item.path.display());

    if Args::cmd().absolute_paths {
        return path;
    }

    // use .\ prefix, otherwise it will look like /usr - absolute path in unix
    let removed = Args::get().start_dir.len();
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

fn needs_showing(node: &Node, path: &str) -> bool {
    // Hide files
    if Args::cmd().hide_files && node.is_file() {
        if Args::cmd().verbose {
            println!("Hiding {path} file because arguments say to hide files | {node:?}");
        }
        return false;
    }

    // Hide directories, but it is still walked
    if Args::cmd().hide_directories && node.is_directory() {
        if Args::cmd().verbose {
            println!(
                "Hiding {path} directory because arguments say to hide directories | {node:?}"
            );
        }
        return false;
    }

    true
}

fn show(node: &Node, path: &str) {
    if Args::cmd().verbose {
        println!("{path} | {node:?}");
    } else {
        println!("{path}");
    }
}
