use std::fs::read_to_string;

pub fn read_lines(filename: String) -> Vec<String> {
    let mut result = Vec::new();
    let entire_file = read_to_string(filename);

    for line in entire_file.unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
