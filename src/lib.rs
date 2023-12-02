pub fn load_lines(path: &str) -> Vec<String> {
    let content = std::fs::read_to_string(path).unwrap();
    content.trim_end().lines().map(String::from).collect()
}
