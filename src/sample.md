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

$a = @(
    ".",
    "--injected",
    ".\.git\",
    ".\.gitignore"
)

.\target\debug\walker.exe @a

cargo run -- "C:/Users/alexko/Downloads"
cargo run -- -p "." -fd --injected ".\.git\" ".\.gitignore" # how to specify vector of arguments?
cargo run -- -p "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/"
cargo run -- -p "d:\" -d1 -c

# WTF here
cargo run -- -p "C:/" -vd1
cargo run -- -p "C:/" -wvd1
```
