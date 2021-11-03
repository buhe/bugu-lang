#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub setp1); // syntesized by LALRPOP

#[test]
fn setp1() {
    assert!(setp1::TermParser::new().parse("22").is_ok());
    assert!(setp1::TermParser::new().parse("(22)").is_ok());
    assert!(setp1::TermParser::new().parse("((((22))))").is_ok());
    assert!(setp1::TermParser::new().parse("((22)").is_err());
}

fn main() {
    println!("Hello, world!");
}
