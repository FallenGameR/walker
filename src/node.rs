use crate::args::Args;
use std::{ path::PathBuf };

#[derive(Debug)]
pub struct Node {
    /// Path reported by fs::ReadDir, see https://doc.rust-lang.org/stable/std/fs/struct.ReadDir.html
    pub path: PathBuf,

    /// The depth of the node relative to the root that has depth of 0
    pub depth: usize,
}

impl Node {
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

        Some(Node {
            path: path,
            depth: 0,
        })
    }
}
