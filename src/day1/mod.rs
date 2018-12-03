use std::io::Read;
use std::fs::File;

pub fn main(){
    let mut file = File::open("./input.txt").expect("file not found");
    let mut strings = String::new();
    file.read_to_string(&mut strings).expect("something went wrong");
    let answer = strings.split_whitespace().fold(0, |mut acc: i32, curr: &str|{
        acc += curr.parse::<i32>().unwrap();
            acc
    });
}
