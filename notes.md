# Sample

> spellchecker: disable

```text
Fast folder walker to be used as replacement for the default fzf walker

Usage: walker.exe [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to start from (current folder by default)

Options:
  -i, --injected <INJECTED>    ** List of injected entries (favorites)
  -e, --excluded <EXCLUDED>    ** List of excluded entries (just the name, it can match any part of the path)
  -m, --max-depth <MAX_DEPTH>  ** Maximum depth of traversal, unlimited by default, starts with 1
  -l, --traverse-links         ** Traverse directory symbolic links
  -f, --hide-files             Exclude files from the output (cdf / codef)
  -d, --hide-directories       Exclude directories from the output (cdf / codef)
  -D, --show-dots              ** Add entries that start with dot (hidden on unix systems)
  -H, --show-hidden            ** Add entries with hidden NTFS attribute  (hidden on windows systems)
      --depth-first-search     ** List the most deep entries first
  -a, --absolute-paths         Use absolute paths, don't trim the output
  -v, --verbose                Verbose output for debugging
  -h, --help                   Print help information
  -V, --version                Print version information
```

```ps1
# injections work
cargo run -- "." -fd -I ".\.git\" -I ".\.gitignore"
cargo run -- "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/" -e "TestResults"
cargo run -- "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/" -e "TestResults" -e src -Ra
cargo run -- "d:\" -m1 -Rfa

cargo run -- ~\Documents\Powershell\
cargo run -- ~\Documents\Powershell\Modules -f

# WTF here (old walkdir implementation)
cargo run -- -p "C:/" -vd1
cargo run -- -p "C:/" -wvd1
```

tests
OneDriveFolder
Drive root
regular folder



.\CoreXTAutomation | Node { 
    path: "C:\\Users\\alexko\\Documents\\Powershell\\Modules\\CoreXTAutomation", 
    depth: 1, 
    metadata: Metadata { 
        file_type: FileType(FileType { attributes: 1040, reparse_tag: 2684354563 }), 
        is_dir: false, 
        is_file: false, 
        permissions: Permissions(FilePermissions { attrs: 1040 }), 
        modified: Ok(SystemTime { intervals: 133076504840750473 }), accessed: Ok(SystemTime { intervals: 133076504840750473 }), created: Ok(SystemTime { intervals: 133076504840750473 }), .. } }
