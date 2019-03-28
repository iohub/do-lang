
mod ast;
mod env;
mod semantic;
mod codegen;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use crate::semantic::semantic_check;
use crate::grammar::ModuleParser;

fn main() {

}
