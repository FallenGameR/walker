# Notes

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

$a = cargo run -- d:\src\golds\pf\

cargo run -- ~\Documents\Powershell\
cargo run -- ~\Documents\Powershell\Modules -f
cargo run -- d:\src\mv -m1 -aHD -e ".git"


cargo build -r
hyperfine.exe ".\target\release\walker.exe d:\src\golds\pf\" # 3.3s wide
cargo run -- d:\src\golds\pf\ -f


```

tests
OneDriveFolder
Drive root
regular folder

fzf << walker << codef
add parrallell reading and breadth-first

- Try without any logic applied - just plain iteration. I could have added something costly.
- Test if jwalk is faster.
- Why printf changes allocated threads? Should regular threads be used instead?
