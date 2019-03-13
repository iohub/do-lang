
mod ast;
mod parser;
mod typ;
mod env;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);


fn main() {
    println!("Hello, world!");
}
