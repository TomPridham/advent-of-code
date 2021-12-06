#![allow(dead_code)]
#![allow(unused_imports)]
mod day1;
mod day2;

use crate::day1::{calculate_depth, calculate_sliding_depth};
use crate::day2::{calculate_final_course, calculate_final_course_with_aim};
fn main() {
    calculate_final_course_with_aim();
}
