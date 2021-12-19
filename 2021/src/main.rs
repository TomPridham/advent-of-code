#![feature(int_abs_diff)]
#![feature(iter_intersperse)]
#![allow(dead_code)]
#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use crate::day1::{calculate_depth, calculate_sliding_depth};
use crate::day2::{calculate_final_course, calculate_final_course_with_aim};
use crate::day3::{calculate_life_support, calculate_power_consumption};
use crate::day4::{find_losing_board, find_winning_board};
use crate::day5::{find_danger_vents, find_danger_vents_diag};
use crate::day6::{model_lanternfish_growth, model_lanternfish_growth_naive};
use crate::day7::{find_optimal_crab_depth_constant, find_optimal_crab_depth_linear};

fn main() {
    find_optimal_crab_depth_linear();
}
