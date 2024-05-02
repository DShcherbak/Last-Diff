use std::fs;

pub fn parse_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    contents
}