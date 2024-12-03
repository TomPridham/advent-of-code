use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
