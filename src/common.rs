use std::fs;

pub fn gather_data(location: &str) -> String {
    fs::read_to_string(location).unwrap()
}

pub fn organize_data_into_lines(raw_data: &str) -> Vec<&str> {
    let organized_data: Vec<&str> = raw_data.split("\n").collect();
    organized_data
}
