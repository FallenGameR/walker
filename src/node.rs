use crate::args::Args;
use std::{
    fs::{self, DirEntry},
    os::windows::prelude::MetadataExt,
    path::PathBuf,
};
use winapi::um::winnt;

#[derive(Debug)]
pub struct Node {
    /// Path reported by fs::ReadDir, see https://doc.rust-lang.org/stable/std/fs/struct.ReadDir.html
    pub path: PathBuf,

    /// The depth of the node relative to the root that has depth of 0
    pub depth: usize,

    /// File metadata that we get cheaply in Windows and it is the correct way to handle
    /// hidden attributes and OneDrive folders since they are reported as reparse points.
    /// Standart rust library is not very helpful here, see https://github.com/rust-lang/rust/issues/46484
    metadata: fs::Metadata,
}

impl Node {
    pub fn new(entry: Result<DirEntry, std::io::Error>, depth: usize) -> Option<Node> {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                if Args::cmd().verbose {
                    eprintln!("ERR: failed to open file system entry, error {:?}", error);
                }
                return None;
            }
        };

        let meta = match entry.metadata() {
            Ok(meta) => meta,
            Err(error) => {
                if Args::cmd().verbose {
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
            depth,
            metadata: meta,
        })
    }

    pub fn new_root(path: PathBuf, meta: fs::Metadata) -> Node {
        Node {
            path: path,
            depth: 0,
            metadata: meta,
        }
    }

    pub fn new_injected(path: &str) -> Option<Node> {
        let path = PathBuf::from(path);
        if !path.exists() {
            if Args::cmd().verbose {
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
                if Args::cmd().verbose {
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

    pub fn is_file(&self) -> bool {
        self.metadata.is_file()
    }

    pub fn is_link(&self) -> bool {
        self.metadata.is_symlink()
    }

    /// Returns true if and only if this entry points to a directory.
    /// self.metadata.is_dir() is buggy for OneDrive folders:
    /// https://github.com/rust-lang/rust/issues/46484
    pub fn is_directory(&self) -> bool {
        self.metadata.file_attributes() & winnt::FILE_ATTRIBUTE_DIRECTORY != 0
    }

    pub fn is_dot(&self) -> bool {
        if let Some(name) = self.path.file_name() {
            name.to_string_lossy().starts_with(".")
        }
        else {
            false
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.metadata.file_attributes() & winnt::FILE_ATTRIBUTE_HIDDEN != 0
    }

    //pub fn is_system(&self) -> bool {
    //    self.metadata.file_attributes() & winnt::FILE_ATTRIBUTE_SYSTEM != 0
    //}
}
