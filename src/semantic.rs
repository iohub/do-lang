use crate::ast::*;
use crate::env::*;

pub fn check(m: Vec<AstNode>) {
    let mut ev = Env::new();
    for node in m {
        check_fndecl(&mut ev, node);
    }
}

fn check_fndecl(ev: &mut Env, n: AstNode) {
    if let AstNode::FnDecl(ident, param, block) = n {
        let proto = ident.to_string() + &join_param(param);
        if let AstNode::Ident(_, typ) = *ident {
            ev.global_def(proto, typ);
        }
        println!("{}", ev);
    }
}

fn join_param(p: Vec<AstNode>) -> String {
    let mut typs = Vec::new();
    for item in p {
        if let AstNode::Ident(_, typ) = item {
            typs.push(typ.to_string());
        }
    }
    return typs.concat();
}
