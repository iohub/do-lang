


#[test]
fn expr_test() {
    use crate::ast::*;
    use crate::grammar::StatementParser;

    let mut sources = "var1 != \"10\";";
    println!("{}", sources);
    StatementParser::new().parse(sources).unwrap();

    sources = "cal(var1, var2);";
    println!("{}", sources);
    StatementParser::new().parse(sources).unwrap();

    sources = r#"
        fn eq(a: int, b: int) -> bool {
            if a > 10 {
                a = a + 100;
            }
            a == b
        }
    "#;
    println!("{}", sources);
    StatementParser::new().parse(sources).unwrap();
    sources = r#"
        fn eq(a: int, b: int) {
            if a > 100 {
                a = b;
                add(a, b);
            }
            a == b
        }
    "#;
    println!("{}", sources);
    StatementParser::new().parse(sources).unwrap();
}
