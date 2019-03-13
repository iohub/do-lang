
mod ast;
mod parser;
mod env;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);


fn main() {
    println!("Hello, world!");
}
