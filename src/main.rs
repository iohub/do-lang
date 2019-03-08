
mod ast;
mod parser;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);


fn main() {
    println!("Hello, world!");
}
