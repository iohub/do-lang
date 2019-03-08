


#[test]
fn expr_test() {
    use crate::ast::*;
    use crate::grammar::ExprParser;

    let sources = "var1 + 10";
    println!("{}", sources);
    ExprParser::new().parse(sources).unwrap();
}
