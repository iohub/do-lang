

pub type StmtBlock = Vec<AstNode>;
pub type Param = Vec<AstNode>;

#[derive(Debug, Clone)]
pub struct Module {
    pub body: StmtBlock,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Int(i32),
    Float(f32),
    Str(String),
    Eof,

    // name: String, typeid: i32
    Ident(String),
    // Fn: Identifer, param: Vec<Identifer>, rtype: Ident, block<Statement>
    FnDecl(Box<AstNode>, Param, Box<AstNode>, StmtBlock),
    // Fn: Identifer, param: Vec<Identifer>
    FnCall(Box<AstNode>, Param),
    BinaryOp(Box<AstNode>, Operator, Box<AstNode>),
    UnaryOp(Operator, Box<AstNode>),
    VarDecl(Box<AstNode>, Box<AstNode>),
    Assignment(Box<AstNode>, Box<AstNode>),
    // conditional, block
    WhileStmt(Box<AstNode>, StmtBlock),
    // conditional, T-block, F-block
    IfStmt(Box<AstNode>, StmtBlock, StmtBlock),
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


pub fn typeof_operator(op: String) -> Operator {
    match &op[..] {
         "||" => Operator::OpOr,
         "=" => Operator::OpAssign,
         "&&" => Operator::OpAnd,
         "==" => Operator::OpEq,
         "!=" => Operator::OpNe,
         "<=" => Operator::OpLe,
         ">=" => Operator::OpGe,
         "<" => Operator::OpLt,
         ">" => Operator::OpGt,
         "!" => Operator::OpNot,

         _ => Operator::OpUnknown,
    }
}
