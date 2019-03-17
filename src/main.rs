
mod ast;
mod env;
mod semantic;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use crate::semantic::semantic_check;
use crate::grammar::ModuleParser;

fn main() {
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
    let nodes = ModuleParser::new().parse(sources).unwrap();
    semantic_check(nodes);
}
