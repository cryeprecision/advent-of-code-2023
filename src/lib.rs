use std::path::PathBuf;

/// Read the file at `./input/<filename>` into a string and then leak the memory
pub fn load_input(filename: &str) -> &'static str {
    let path: PathBuf = ["./input", filename].iter().collect();
    std::fs::read_to_string(path)
        .expect("input file should exist and contain valid utf-8")
        .leak()
}
