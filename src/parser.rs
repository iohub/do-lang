


#[test]
fn expr_test() {
    use crate::ast::*;
    use crate::grammar::StatementParser;

    let mut sources = r#"
        fn eq(a: int, b: int) -> bool {
            if a > 10 {
                a = a + 100 + calc(a, bval);
            }
            a == b
        }
    "#;
    println!("{}", sources);
    StatementParser::new().parse(sources).unwrap();
}


#[test]
fn module_test() {
    use crate::ast::*;
    use crate::grammar::ModuleParser;
    let sources = r#"
        fn eq(a: int, b: int) {
            let c = a + 1001;
            let d;
            if a > 100 {
                a = b + 1000 + c + d;
                add(a, b);
            }
            a == b
        }
        let a = b + 1000 + c + d;
        
        fn main() {
            a = calc(a10, val1) + d + calc(a, val100);
            while a > b + 100 + c {
                let b = a + calc(v1, v2);
            }
        }
    "#;
    println!("{}", sources);
    let nodes = ModuleParser::new().parse(sources).unwrap();
    println!("node:{}", nodes.len());
    display_module(nodes);
}