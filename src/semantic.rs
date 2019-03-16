use crate::ast::*;
use crate::env::*;

pub fn check(m: Vec<AstNode>) {
    let mut ev = Env::new();
    for node in m {
        println!("{}", node);
        match node {
            AstNode::FnDecl(_, _, _) => check_fndecl(&mut ev, node),
            AstNode::VarDecl(_, _, _) => check_vardecl(&mut ev, node, true),
            _ => (),
        }
    }
}

fn prototype_fn(ident: String, p: &Vec<AstNode>) -> String {
    ident + &join_param(p)
}

fn check_fndecl(ev: &mut Env, n: AstNode) {
    if let AstNode::FnDecl(ident, param, block) = n {
        let proto = prototype_fn(ident.to_string(), &param);
        if let AstNode::Ident(_, typ) = *ident {
            ev.global_def(&proto, typ);
            ev.enter_scope();
            define_local_var(ev, &param);
        }
        check_stmtblock(ev, &block);
        ev.leave_scope();
    }
}

fn check_stmtblock(ev: &mut Env, block: &Vec<AstNode>) {
    for stmt in block {
        match stmt {
            AstNode::VarDecl(var, val, typ) => {
                println!("{}-{}-{}", var, val, typ);
                check_vardecl(ev, stmt.clone(), false);
                println!("{}", ev);
            }
            AstNode::Assignment(_, _) => {
                check_assignstmt(ev, stmt.clone());
            }
            _ => (),
        }
    }
}

fn check_assignstmt(ev: &mut Env, n: AstNode) {
    if let AstNode::Assignment(var, valexpr) = n {
        if let AstNode::Ident(vname, _) = *var {
            println!("resolve: {}", vname);
            let ltyp = ev.resolve(&vname).unwrap();
            let rtyp = typeof_valexpr(ev, &*valexpr);
            if ltyp != AstType::Unknown && ltyp != rtyp {
                panic!("unmatch {} {}", ltyp, rtyp);
            }
        }
    }
}

fn check_vardecl(ev: &mut Env, n: AstNode, global: bool) {
    if let AstNode::VarDecl(var, val, typ) = n.clone() {
        if let AstNode::Ident(vname, _) = *var {
            if ev.can_resolve(&vname) {
                panic!("redefine '{}'", vname);
            }
            if global { ev.global_def(&vname, typ); }
            else { ev.local_def(&vname, typ); }
            let vtyp = typeof_valexpr(ev, &val);
            ev.update(&vname, vtyp);
        }
    }
}

fn typeof_valexpr(ev: &mut Env, n: &AstNode) -> AstType {
    match n {
        AstNode::BinaryOp(lhs, op, rhs, typ) => {
            typeof_binary_op(ev, n.clone())
        },
        _ => typeof_valobj(ev, n.clone()),
    }
}

fn typeof_valobj(ev: &mut Env, n: AstNode) -> AstType {
    match n {
        AstNode::Int(_) => AstType::Int,
        AstNode::Float(_) => AstType::Float,
        AstNode::Str(_) => AstType::Str,
        AstNode::Ident(var, typ) => {
            if !ev.can_resolve(&var) {
                panic!("cann't resolve {}", var);
            }
            ev.resolve(&var).unwrap()
        }
        AstNode::FnCall(ident, param) => {
            let proto = prototype_fn(ident.to_string(), &param);
            match ev.global_resolve(&proto) {
                Some(typ) => typ.clone(),
                None => panic!("cann't resolve fn proto:{}", proto),
            }
        },
        AstNode::Nil => AstType::Unknown,
        _ => panic!("unexpected astnode:{}", n),
    }
}

fn typeof_binary_op(ev: &mut Env, n: AstNode) -> AstType {
    if let AstNode::BinaryOp(lhs, op, rhs, typ) = n {
        if !is_match_op(op) {
            panic!("unexpected operator:{}", op);
        }
        let rtyp = typeof_valobj(ev, *rhs);
        let ltyp = match *lhs {
            AstNode::BinaryOp(_, _, _, _) => typeof_binary_op(ev, *lhs),
            _ => typeof_valobj(ev, *lhs),
        };
        if rtyp != ltyp {
            panic!("unexpected {} == {}", ltyp, rtyp);
        }
        return rtyp;
    }
    return AstType::Unknown;
}

fn is_match_op(op: Operator) -> bool {
    match op {
        Operator::OpPlus | Operator::OpSub |
        Operator::OpMul | Operator::OpDiv => true,
        _ => false,
    }
}

fn define_local_var(ev: &mut Env, p: &Vec<AstNode>) {
    for var in p {
        if let AstNode::Ident(name, typ) = var {
            ev.local_def(&name, typ.clone());
        }
    }
}

fn join_param(p: &Vec<AstNode>) -> String {
    let mut typs = Vec::new();
    for item in p {
        if let AstNode::Ident(_, typ) = item {
            typs.push(typ.to_string());
        }
    }
    return typs.concat();
}

#[test]
fn module_test() {
    use crate::ast::*;
    use crate::semantic::check;
    use crate::grammar::ModuleParser;
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
    println!("{}", sources);
    let nodes = ModuleParser::new().parse(sources).unwrap();
    check(nodes);
}
