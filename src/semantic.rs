use crate::ast::*;
use crate::env::*;

pub fn semantic_check(stmt: Vec<AstNode>) {
    let mut ev = Env::new();
    for node in stmt {
        println!("[check]:{}", node);
        match node {
            AstNode::FnDecl(_, _, _) => check_fndecl(&mut ev, node),
            AstNode::VarDecl(_, _, _) => check_vardecl(&mut ev, node, true),
            _ => (),
        }
    }
}

fn prototype_fn(ev: &mut Env, ident: String, p: &Vec<AstNode>) -> String {
    ident + &join_param(ev, p)
}

fn ident_name(ident: &AstNode) -> String {
    match ident {
        AstNode::Ident(var, _) => var.clone(),
        _ => "fuck".to_string(),
    }
}

fn check_fndecl(ev: &mut Env, n: AstNode) {
    if let AstNode::FnDecl(ident, param, block) = n {
        let proto = prototype_fn(ev, ident_name(&ident), &param);
        if ev.global_has(&proto) { panic!("redefine function:{}", proto) }
        if let AstNode::Ident(_, typ) = *ident {
            ev.global_def(&proto, typ);
            ev.enter_scope();
            define_local_var(ev, &param);
        }
        check_stmtblock(ev, &block);
        println!("[leave_scope]:{}", ev);
        ev.leave_scope();
    }
}

fn check_stmtblock(ev: &mut Env, block: &Vec<AstNode>) {
    for stmt in block {
        match stmt {
            AstNode::VarDecl(_, _, _) => { check_vardecl(ev, stmt.clone(), false); }
            AstNode::Assignment(_, _) => { check_assignstmt(ev, stmt.clone()); },
            AstNode::IfStmt(cond, tblock, fblock) => {
                if typeof_bool_expr(ev, &cond.clone()) == AstType::Unknown {
                    panic!("typeof_bool_expr");
                }
                check_stmtblock(ev, tblock);
                check_stmtblock(ev, fblock);
            }
            AstNode::WhileStmt(cond, block) => {
                if typeof_bool_expr(ev, &cond.clone()) == AstType::Unknown {
                    panic!("typeof_bool_expr");
                }
                check_stmtblock(ev, block);
            }
            _ => {
                if typeof_bool_expr(ev, &stmt.clone()) == AstType::Unknown {
                    panic!("typeof_bool_expr");
                }
            }
        }
    }
}

fn check_assignstmt(ev: &mut Env, n: AstNode) {
    if let AstNode::Assignment(var, valexpr) = n {
        if let AstNode::Ident(vname, _) = *var {
            let ltyp = ev.resolve(&vname).unwrap();
            let rtyp = typeof_valexpr(ev, &*valexpr);
            if ltyp == AstType::Unknown { ev.update(&vname, rtyp) }
            else if ltyp != rtyp {
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
        AstNode::BinaryOp(_, op, _, _) => {
            if !is_math_op(*op) { panic!("unmatch math Operator{}", op); }
            typeof_binary_op(ev, n.clone())
        },
        _ => typeof_valobj(ev, n),
    }
}

fn typeof_bool_expr(ev: &mut Env, n: &AstNode) -> AstType {
    match n {
        AstNode::BinaryOp(_, op, _, _) => {
            if !is_logic_op(*op) { panic!("unmatch logic Operator{}", op); }
            typeof_binary_op(ev, n.clone())
        },
        _ => typeof_valobj(ev, n),
    }
}


fn typeof_valobj(ev: &mut Env, n: &AstNode) -> AstType {
    match n {
        AstNode::Int(_) => AstType::Int,
        AstNode::Float(_) => AstType::Float,
        AstNode::Str(_) => AstType::Str,
        AstNode::Ident(var, _) => {
            if !ev.can_resolve(&var) {
                panic!("cann't resolve {}", var);
            }
            ev.resolve(&var).unwrap()
        }
        AstNode::FnCall(ident, param) => {
            let proto = prototype_fn(ev, ident_name(&ident), &param);
            match ev.global_resolve(&proto) {
                Some(typ) => typ.clone(),
                None => panic!("cann't resolve fn proto:{}", proto),
            }
        },
        AstNode::BinaryOp(_, _, _, _) => typeof_valexpr(ev, n),
        AstNode::Nil => AstType::Unknown,
        _ => panic!("unexpected astnode:{}", n),
    }
}

fn typeof_binary_op(ev: &mut Env, n: AstNode) -> AstType {
    if let AstNode::BinaryOp(lhs, _, rhs, _) = n {
        let rtyp = typeof_valobj(ev, &*rhs);
        let ltyp = match *lhs {
            AstNode::BinaryOp(_, _, _, _) => typeof_binary_op(ev, *lhs),
            _ => typeof_valobj(ev, &*lhs),
        };
        if rtyp != ltyp {
            panic!("unexpected {} == {}", ltyp, rtyp);
        }
        return rtyp;
    }
    panic!("cann't reach here");
}

fn is_math_op(op: Operator) -> bool {
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

fn typeof_param(ev: &mut Env, n: AstNode) -> AstType {
    let mut rtyp = AstType::Unknown;
    if let AstNode::Ident(_, typ) = n {
        match typ {
            AstType::Ext(name) => {
                rtyp = ev.resolve(&name).unwrap();
            },
            _ => rtyp = typ
        }
    }
    rtyp
}

fn join_param(ev: &mut Env, p: &Vec<AstNode>) -> String {
    let mut typs = Vec::new();
    let mut typ: AstType;
    for item in p {
        match item {
            AstNode::Ident(_, _) => { typ = typeof_param(ev, item.clone()); },
            _ => { typ = typeof_valexpr(ev, item); }
        }
        typs.push(typ.to_string());
    }
    return typs.concat();
}

#[test]
fn module_test() {
    use crate::ast::*;
    use crate::semantic::*;
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
    semantic_check(nodes);
}
