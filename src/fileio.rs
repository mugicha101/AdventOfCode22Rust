use std::fs;

pub fn input(id: &str) -> Vec<String> {
    let content = fs::read_to_string(id)
        .expect("file read failed");
    let output: Vec<String> = content.lines().map(str::to_string).collect();
    return output;
}