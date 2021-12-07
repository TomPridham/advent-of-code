#![allow(dead_code)]
#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;
mod day4;

use crate::day1::{calculate_depth, calculate_sliding_depth};
use crate::day2::{calculate_final_course, calculate_final_course_with_aim};
use crate::day3::{calculate_life_support, calculate_power_consumption};
use crate::day4::{find_losing_board, find_winning_board};

fn main() {
    find_losing_board();
}
