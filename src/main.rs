
mod ast;
mod env;
mod semantic;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);


fn main() {
    println!("Hello, world!");
}
