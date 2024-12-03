#![allow(dead_code)]
mod day1;
mod day2;
mod day3;
mod read_input;

fn main() {
    let res = day3::get_enabled_muls();
    println!("{res}");
}
