use std::path::Path;

pub fn file_exists(filename: &str) -> bool {
    return Path::new(filename).exists();
}

pub fn is_node_project() -> bool {
    return file_exists("package.json");
}

pub fn is_rust_project() -> bool {
    return file_exists("Cargo.toml");
}

pub fn is_python_project() -> bool {
    file_exists("requirements.txt")
}
