use std::fmt;

pub type StmtBlock = Vec<AstNode>;
pub type Param = Vec<AstNode>;

#[derive(Debug, Clone)]
pub struct Module {
    pub body: StmtBlock,
}

#[derive(Debug, Clone)]
pub enum AstType {
    Int,
    Float,
    Str,
    Bool,
    // TODO: extend type: struct, enum, interface ...
    Ext(String),
    Unknown,
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

#[derive(Debug, Clone)]
pub enum AstNode {
    Int(i32),
    Float(f32),
    Str(String),
    Nil,

    Ident(String, AstType),
    // Fn: Identifer, param: Vec<Identifer>, rtype: Ident, block<Statement>
    FnDecl(Box<AstNode>, Param, StmtBlock, AstType),
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
}

#[derive(Debug, Clone)]
pub enum TypedAst {
    Node(AstNode, AstType)
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn display_module(m: Vec<AstNode>) {
    for n in m {
        println!("{}\n", n)
    }
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


#[test]
fn ast_show_test() {
    let val = AstNode::Int(12);
    println!("node:{}", val);
}
