# Notes

> spellchecker: disable

## Tests

> walker [PATH]                       // Path to start from (current folder by default)

cargo run -- "."
cargo run -- "d:\"
cargo run -- "d:\src\golds\pf\"

> walker -I, --included <INCLUDED>    // List of included entries (favorites) -a doesn't seem to affect these

cargo run -- -I "Included but absent" --included ".vscode" | select -f 10

> walker -e, --excluded <EXCLUDED>    // List of excluded entries (just the name, it can match any part of the path)

cargo run -- -e target

> walker -R, --show-root              // Include root folder to the output

cargo run -- -e target -R

> walker -m, --max-depth <MAX_DEPTH>  // Maximum depth of traversal, unlimited by default, children of root has depth 1

cargo run -- -m 1

> walker -l, --dont-traverse-links    // Do not traverse directory symbolic links

cargo run -- ~\Documents\Powershell\Modules -e pstoolset -l

> walker -f, --hide-files             // Hide files from the output (cdf / codef)

cargo run -- -f -m 2

> walker -d, --hide-directories       // Hide directories from the output, but they are still walked (cdf / codef)

cargo run -- -d -m 2

> walker -d, --hide-directories       // Hide directories from the output, but they are still walked (cdf / codef)

cargo run -- -d -m 2

> walker -D, --show-dots              // Add entries that start with dot (hidden on unix systems)

cargo run -- -D -m 2

> walker -a, --absolute-paths         // Use absolute paths, don't trim the output

cargo run -- -R -e target -a




## Performance

```ps1
# NOTE: Reading metadata (windows hidden flag and symlink disambiguation) is very expensive
cargo build -r
hyperfine.exe ".\target\release\walker.exe $pfgold" # 3s wide
$a = cargo run -- d:\src\golds\pf\ # -f
```
