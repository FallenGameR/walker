pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.chars().map(|c| match c {
        '/' => std::path::MAIN_SEPARATOR,
        _ => c,
    });
    path.collect()
}
