


#[test]
fn module_test() {
    use crate::ast::*;
    use crate::grammar::ModuleParser;
    let sources = r#"
        fn foo1(a: int, b: int) -> int {
            let c = a + 1001;
            let d;
            if a > 100 {
                d = b + 1000 + c + a;
            }
            a
        }
        fn foo2(a: int) -> bool {
            a == 100
        }

        let a = 1000 + 10;
        fn main() {
            let b;
            a = foo1(a, 1001) + 123 + foo1(a, 100+101);
            b = foo1(123, a);
            while a > b + 100 {
                b = a + foo1(a, b);
            }
        }
    "#;
    println!("{}", sources);
    let nodes = ModuleParser::new().parse(sources).unwrap();
    display_module(nodes);
}
