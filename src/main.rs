
mod ast;
mod env;
mod semantic;
mod codegen;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);


extern crate clap;
use clap::{Arg, App};

use crate::semantic::semantic_check;
use crate::grammar::ModuleParser;
use crate::codegen::LLVMGenerator;

fn main() {
    let matches = App::new("do language")
        .arg(Arg::with_name("source")
        .short("s")
        .takes_value(true))
        .get_matches();
    let fname = matches.value_of("source").unwrap();

    let contents = std::fs::read_to_string(fname)
        .expect("[error] read_to_string");
    let stmts = ModuleParser::new().parse(&contents.to_string()).unwrap();
    let typed_ast = semantic_check(stmts);
    unsafe {

    let mut generator = LLVMGenerator::new();
    generator.run(&fname.to_string(), &typed_ast);

    }

}
