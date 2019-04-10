use std::fmt;

pub type StmtBlock = Vec<AstNode>;
pub type Param = Vec<AstNode>;

#[derive(Debug, Clone)]
pub struct Module {
    pub body: StmtBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Int,
    Float,
    Str,
    Bool,
    // TODO: extend type: struct, enum, interface ...
    Ext(String),
    Nil,
    Undef,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Int(i32),
    Float(f32),
    Str(String),
    Nil,

    Ident(String, AstType),
    // Fn: Identifer, param: Vec<Identifer>, rtype: Ident, block<Statement>
    FnDecl(Box<AstNode>, Param, StmtBlock),
    // Fn: Identifer, param: Vec<Identifer>
    FnCall(Box<AstNode>, Param),
    BinaryOp(Box<AstNode>, Operator, Box<AstNode>, AstType),
    UnaryOp(Operator, Box<AstNode>),
    VarDecl(Box<AstNode>, Box<AstNode>, AstType),
    Assignment(Box<AstNode>, Box<AstNode>),
    // conditional, block
    WhileStmt(Box<AstNode>, StmtBlock),
    // conditional, T-block, F-block
    IfStmt(Box<AstNode>, StmtBlock, StmtBlock),
    ReturnStmt(Box<AstNode>, AstType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    OpOr,
    OpAssign,
    OpAnd,
    OpEq,
    OpNe,
    OpGe,
    OpLe,
    OpGt,
    OpLt,
    OpNot,
    OpPlus,
    OpSub,
    OpMul,
    OpDiv,
    OpUnknown,
}

pub fn typeof_ident(v: &String) -> AstType {
    let v2 = v.to_lowercase();
    match &v2[..] {
        "int" => AstType::Int,
        "float" => AstType::Float,
        "str" => AstType::Str,
        "bool" => AstType::Bool,
        _ => AstType::Ext(v2),
    }
}

pub fn ident_name(ident: &AstNode) -> String {
    match ident {
        AstNode::Ident(var, _) => var.clone(),
        _ => "fuck".to_string(),
    }
}

pub fn ident_type(ident: &AstNode) -> AstType {
    match ident {
        AstNode::Ident(_, typ) => typ.clone(),
        _ => unreachable!(),
    }
}

pub fn update_ident_type(ident: &mut AstNode, typ: AstType) {
    if let AstNode::Ident(_, ref mut _typ) = ident {
        *_typ = typ;
    }
}

pub fn nil_node(n: &AstNode) -> bool {
    match n {
        AstNode::Nil => true,
        _ => false,
    }
}

impl fmt::Display for AstType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn is_logic_op(op: Operator) -> bool {
    match op {
        Operator::OpEq | Operator::OpNe | Operator::OpLe | Operator::OpGe |
        Operator::OpLt | Operator::OpGt | Operator::OpOr | Operator::OpAnd => true,
        _ => false,
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Operator::OpOr => "||",
            Operator::OpAssign => "=",
            Operator::OpAnd => "&&",
            Operator::OpEq => "==",
            Operator::OpNe => "!=",
            Operator::OpGe => ">=",
            Operator::OpLe => "<=",
            Operator::OpGt => ">",
            Operator::OpLt => "<",
            Operator::OpNot => "!",
            Operator::OpPlus => "+",
            Operator::OpSub => "-",
            Operator::OpMul => "*",
            Operator::OpDiv => "/",
            _ => "UnKnown",
        };
        s.fmt(f)
    }
}


