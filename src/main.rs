mod args;
mod node;
use args::{Args, ARGS};
use jwalk::{Error, WalkDirGeneric};
use node::Node;
use std::{path::{Path}};

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
        println!("{path}");
    }

    // Start walking from the start directory
    jwalk(&Args::get().start_dir).unwrap(); // 0.8s for PfGold
}

fn jwalk<P: AsRef<Path>>(root: P) -> Result<(), Error> {
    // Construct walker
    let walker = WalkDirGeneric::<((), String)>::new(root)
        .max_depth(Args::get().max_depth)
        .follow_links(!Args::cmd().dont_traverse_links)
        .skip_hidden(!Args::cmd().show_dots);

    // Children processing
    let excluded = &Args::get().excluded;
    let walker = walker.process_read_dir(|_, _, _, children| {
        // Don't retain excluded; that makes excluded not traversible as well
        children.retain(|item| {
            match item {
                Ok(entry) => !excluded.contains(&entry.file_name.to_ascii_lowercase()),
                Err(____) => false,
            }
        });

        // Prepare output line
        for item in children.iter_mut() {
            if let Ok(entry) = item {
                entry.client_state = trim(entry.path().display());
            }
        };
    });

    // Root is rendered separatelly
    let mut iter = walker.into_iter();
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

        if show_file || show_dir {
            println!("{}", entry.path().display());//entry.client_state);
        }
    }

    Ok(())
}

fn trim(path: std::path::Display) -> String {
    let path = path.to_string();

    if Args::cmd().absolute_paths {
        return path;
    }

    path
    //// use .\ prefix, otherwise it will look like /usr - absolute path in unix
    //let removed = Args::get().start_dir.len();
    //let remaining = path.len() - removed;
    //let mut result = String::with_capacity(2 + remaining);
    //result.push('.');
    //result.push(std::path::MAIN_SEPARATOR);
    //result.push_str(path.split_at(removed).1);
    //result
}

pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}

