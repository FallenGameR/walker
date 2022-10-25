use crate::args::Args;
use std::{fs, os::windows::prelude::*};
use walkdir::DirEntry;

pub fn accept_path(args: &Args, item: &DirEntry) -> bool {
    if args.verbose {
        println!("- {:?}", item.file_name().to_str());
    }

    if !args.add_files && is_file(item) {
        return false;
    }

    if !args.add_dots && is_dot(item) {
        return false;
    }

    if !args.add_hidden && is_hidden(item) {
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
    if let Ok(meta) = fs::metadata(&item.path()) {
        return (meta.file_attributes() & 0x2) > 0
    }
    false
}
