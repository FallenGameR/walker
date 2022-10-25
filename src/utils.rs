pub fn normalize(path: std::path::Display) -> String {
    let path = path.to_string();
    let path = path.replace("\\", "/");
    path
}