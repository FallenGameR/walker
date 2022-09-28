use std::fs;

fn main() {
    for file in fs::read_dir("C:/Users/alexko/Downloads").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
