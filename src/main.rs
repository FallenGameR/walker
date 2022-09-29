//extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    // into_iter does recursive walk
    // traverses alphabetically even when .contents_first(true) is called
    for file in WalkDir::new("C:/Users/alexko/Downloads").contents_first(true).min_depth(1)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        /*
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
        }
        */
        println!("{}", file.path().display());
    }


    // iter.follow_links(true) - start following symbolic links
}

/*
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
*/

/*
rust analog requirements:
- work with cyrillic folders
- work with OneDrive folders / same_file_system
- support excluded folders (env vars / settings file)
- support both folder (cdf) and file and folder (codef) scenarios
- be fast
- work on relative paths (normalization)
- traverse files first then drill into folders
- support quick access (env vars) - add it to the end of list???
- min/max depth (arguments / settings file)
- folders will be returned first then their files (walker)
- current folder . auto added for (codef) scenario
- follow_links by default but can be omited

settings:
- args - easy direct overrides, win over anything
- env variables? yes, easy to change behaviour between the systems
- settings file that is set by env variable, useful defaults

arguments:
- let args: Vec<String> = env::args().collect();


https://docs.rs/clap/latest/clap/
https://github.com/clap-rs/clap/tree/45241277043f2a8cc64230e18574b88b005e765c/examples
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-05-working-with-environment-variables.html
*/