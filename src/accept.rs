use crate::args::Args;
use std::{fs, os::windows::prelude::*};
use walkdir::DirEntry;

//std::path::MAIN_SEPARATOR.

/*
use std::{env, fs};

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

https://doc.rust-lang.org/std/fs/#
https://doc.rust-lang.org/std/fs/fn.read_dir.html
https://doc.rust-lang.org/std/fs/struct.DirEntry.html - metadata is cheap to call, reads from buffer that is populated with lots of entries in the same folder
https://docs.rs/jwalk/latest/jwalk/ - test if parrallelizm is a thing
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html is_symlink_dir
https://doc.rust-lang.org/stable/std/os/windows/fs/trait.MetadataExt.html file_attributes
*/
pub fn accept_path(args: &Args, item: &DirEntry) -> bool {
    if !args.hide_files && is_file(item) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_file", item.path().display());
        }
        return false;
    }

    if !args.show_dots && is_dot(item) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_dot", item.path().display());
        }
        return false;
    }

    if !args.show_hidden && is_hidden(item) {
        if args.verbose {
            eprintln!("dbg> {} - not accepted, is_hidden", item.path().display());
        }
        return false;
    }

    if args.verbose {
        eprintln!("dbg> {} - accepted", item.path().display());
    }
    true
}

fn is_file(item: &DirEntry) -> bool {
    if let Ok(meta) = item.metadata() {
        return meta.is_file()
    }
    false
}

fn is_dot(item: &DirEntry) -> bool {
    item.file_name().to_str().map_or(false, |s| s.starts_with("."))
}

fn is_hidden(item: &DirEntry) -> bool {
    //println!("TEST {}", item.path().display());

    if let Ok(meta) = fs::metadata(&item.path()) {
        //println!("META {}", meta.file_attributes());
        let attributes = meta.file_attributes();
        let hidden = (attributes & 0x2) != 0;
        let system = (attributes & 0x4) != 0;
        let directory = (attributes & 0x16) != 0;

        if hidden {
            // Drive roots have hidden flag set but we need to list them regardless.
            if hidden && system && directory && item.path().parent().is_none() {
                return false
            }
            return true;
        }
    }

    //println!("META no");
    false
}
