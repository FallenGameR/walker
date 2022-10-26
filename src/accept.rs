use crate::args::Args;
use std::{fs, os::windows::prelude::*};
use walkdir::DirEntry;

pub fn accept_path(args: &Args, item: &DirEntry) -> bool {
    if args.verbose {
        println!("- {:?}", item.file_name().to_str());
    }

    if !args.add_files && is_file(item) {
        println!("-- is_file, excluded");
        return false;
    }

    if !args.add_dots && is_dot(item) {
        println!("-- is_dot, excluded");
        return false;
    }

    if !args.add_hidden && is_hidden(item) {
        println!("-- is_hidden, excluded");
        return false;
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
