use pest::iterators::Pair;
use pest::Parser;
use std::path::Path;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct PieceParser;


impl PieceParser {
    pub fn _parse(buf: &str) {
        let _module = PieceParser::parse(Rule::_module, buf)
            .unwrap_or_else(|e| panic!("parser error: {}", e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // TODO: fix init with call()
        let code = r#"
            fn add(a: int, b: int) -> int {
                let c;
                let a = 1001 + 10;
                a = callfn(a, c);
                while a > 100 {
                    a = add(100, "ok");
                }
            }
            if a > 100 {
                undefined = calc("hello world", 12);
                let val0 = 101;
            }
        "#;
        PieceParser::_parse(code);
    }
}