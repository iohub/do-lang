


#[test]
fn module_test() {
    use crate::ast::*;
    use crate::grammar::ModuleParser;
    let sources = r#"
        fn foo1(a: int, b: int) {
            let c = a + 1001;
            let d;
            if a > 100 {
                d = b + 1000 + c + a;
                add(a, d);
            }
            a == b
        }
        fn foo2(a: int) -> bool {
            a == 100
        }

        let a = b + 1000 + c + d;
        fn main() {
            a = calc(a10, "ok") + d + calc(a, 100+101);
            while a > b + 100 + c {
                let b = a + calc(v1, v2);
            }
        }
    "#;
    println!("{}", sources);
    let nodes = ModuleParser::new().parse(sources).unwrap();
    display_module(nodes);
}
