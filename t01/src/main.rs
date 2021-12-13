pub mod one;
pub mod two;
pub mod test;

fn main () {
    one::one_log();
    one::one_handle("hly".to_string());
    two::two_log();
    test::test_log();
}
