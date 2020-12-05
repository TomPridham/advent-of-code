mod day1;
use day1::{fix_expense_report, fix_expense_report_triple};
fn main() {
    assert_eq!(731731, fix_expense_report());
    assert_eq!(116115990, fix_expense_report_triple());
    print!("{}", fix_expense_report_triple())
}
