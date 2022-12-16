mod args;
mod node;
use args::{Args, ARGS};
use crossbeam_channel::unbounded;
use jwalk::{DirEntry, Error, WalkDir, WalkDirGeneric};
use node::Node;
use std::{
    cmp::Ordering, collections::HashSet, ffi::OsString, fs, os::windows::prelude::MetadataExt,
    path::PathBuf, thread,
};
use threadpool::ThreadPool;

fn main() {
    ARGS.set(Args::new()).expect("Could not initialize ARGS");

    // Arguments sanity check
    if Args::get().cmd.hide_files
        && Args::get().cmd.hide_directories
        && !Args::get().cmd.show_root
        && (Args::get().cmd.included.len() == 0)
    {
        eprintln!("ERR: nothing to show, arguments instruct to hide files, directories, root and nothing is injected");
        return;
    }

    // Injections are inserted here
    for node in Args::get()
        .cmd
        .included
        .iter()
        .filter_map(|path| Node::new_injected(&path))
    {
        // Don't trim and ignore -fd flags
        let path = node.path.display().to_string();
        show(&node, &path);
    }

    // Start walking from the start directory
    let path = PathBuf::from(&Args::get().start_dir);

    match fs::metadata(&path) {
        Ok(meta) => {
            let root = Node::new_root(path, meta);
            //walk(root);           // 3s for PfGold
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
    let walk_dir = walk_dir.max_depth(Args::get().max_depth_resolved);
    let walk_dir = walk_dir.follow_links(!Args::cmd().dont_traverse_links);
    let walk_dir = walk_dir.skip_hidden(!Args::cmd().show_dots);

    let excluded = &Args::get().excluded;

    let walk_dir = walk_dir.process_read_dir(|depth, parent, _, children| {
        // Don't retain excluded; that makes excluded not traversible as well
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

    // Root has special meaning
    let mut iter = walk_dir.into_iter();
    let first = iter.nth(0);

    if Args::cmd().show_root {
        if let Some(entry) = first {
            let entry = entry?;
            println!("{}", entry.path().display());
        }
    }

    // Show all the entries
    let show_files = !Args::cmd().hide_files;
    let show_dirs = !Args::cmd().hide_directories;

    for entry in iter {
        let entry = entry?;
        let kind = entry.file_type;
        let show_file = show_files && kind.is_file();
        let show_dir = show_dirs && (kind.is_dir() || kind.is_symlink());
        let show = show_file || show_dir;

        if show {
            println!("{}", entry.path().display());
        }

        //println!(
        //    "d={},f={},s={}|d={},f={},s={} - {}",
        //    entry.file_type.is_dir(),
        //    entry.file_type.is_file(),
        //    entry.file_type.is_symlink(),
        //    meta.is_dir(),
        //    meta.is_file(),
        //    meta.is_symlink(),
        //    entry.path().display());
    }

    Ok(())
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

fn show(node: &Node, path: &str) {
    if Args::cmd().verbose {
        println!("{path} | {node:?}");
    } else {
        println!("{path}");
    }
}
