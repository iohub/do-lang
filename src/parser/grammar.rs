use pest::iterators::Pair;
use pest::Parser;
use std::path::Path;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct PieceParser;


impl PieceParser {
    pub fn _parse(buf: &str) {
        let module = PieceParser::parse(Rule::module, buf)
            .unwrap_or_else(|e| panic!("parser error: {}", e));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = r#"
            fn foo(a: int) -> int {
                if a > 100 {
                    let valb = 1000;
                }
            }
        "#;
        PieceParser::_parse(code);
    }
}