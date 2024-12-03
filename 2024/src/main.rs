#![allow(dead_code)]
mod day1;
mod day2;
mod read_input;

fn main() {
    let res = day2::get_dampened_reports();
    println!("{res}");
}
