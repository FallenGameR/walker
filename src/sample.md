# Sample

> spellchecker: disable

```text
* Fast folder walker to be used as replacement for the default fzf walker

Usage: walker.exe [OPTIONS]

Options:
*  -f, --add-files            * Add files to the output (cdf / codef)
*  -c, --add-current-folder   ? Add the current folder to the output
  -., --add-dots             * Add entries that start with dot to the output (hidden on unix systems)
  -n, --add-hidden           * Add entries with hidden NTFS attribute to the output (hidden on windows systems)
*  -l, --leafs-first          List the most deep entries first
  -t, --link-traversal       Traverse symbolic links
  -d, --depth <DEPTH>        Maximum depth of traversal, unlimited by default
  -p, --path <PATH>          Path to start from (current folder by default)
  -i, --injected <INJECTED>  * List of injected entries (favorites)
  -e, --excluded <EXCLUDED>  * List of excluded entry names
  -h, --help                 Print help information
  -V, --version              Print version information
```

```ps1
$path = "d:/OneDrive/Projects/Coding/CoreXtAutomation/"
$path = "d:/OneDrive/Projects/Coding/Подсветка синтаксиса/"
$path = "C:/Users/alexko/Downloads"

cargo run -- -p $path -t
cargo run -- -p c:\Users\alexko\Documents\Powershell\Modules\ --add-dots
```
