mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use day1::{fix_expense_report, fix_expense_report_triple};
use day2::{find_valid_passwords, find_valid_passwords_positional};
use day3::{tree_collision_detector, tree_collision_detector_v2};
use day4::{strict_validate_passports, validate_passports};
use day5::{find_highest_seat, find_my_seat};

fn main() {
    assert_eq!(731731, fix_expense_report());
    assert_eq!(116115990, fix_expense_report_triple());
    assert_eq!(536, find_valid_passwords());
    assert_eq!(558, find_valid_passwords_positional());
    assert_eq!(145, tree_collision_detector());
    assert_eq!(3424528800, tree_collision_detector_v2());
    assert_eq!(228, validate_passports());
    assert_eq!(175, strict_validate_passports());
    assert_eq!(801.0, find_highest_seat());
    assert_eq!(597.0, find_my_seat());

    println!("{}", find_my_seat())
}
