mod day1;
mod day2;
use day1::{fix_expense_report, fix_expense_report_triple};
use day2::{find_valid_passwords, find_valid_passwords_positional};

fn main() {
    assert_eq!(731731, fix_expense_report());
    assert_eq!(116115990, fix_expense_report_triple());
    assert_eq!(536, find_valid_passwords());
    print!("{}", find_valid_passwords_positional())
}
