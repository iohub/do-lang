
use pest::prelude::*;
use std::collections::VecDeque;
use super::StmtBlock;
use super::Module;
use super::AstNode;
use super::typeof_operator;
use super::Operator;
use super::Param;


impl_rdp! {
    grammar! {
        keyword = @{
            ["fn"] | ["let"] | ["if"] | ["return"] | ["else"] | ["for"] |
            ["false"] | ["true"] | ["while"] | ["loop"] | ["for"] |
            ["break"] | ["continue"] | ["enum"] | ["impl"] | ["use"] |
            ["struct"] | ["pub"] | ["mut"] | ["int"] | ["float"] | ["string"]
        }

        base_type = @{
            ["int"] | ["float"] | ["string"]
        }

        // token define
        comma = { [","] }
        semi = { [";"] }
        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
        alpha = _{ ['a'..'z'] | ['A'..'Z'] | ["_"] }
        alphanumeric = _{ alpha | ['0'..'9'] }
        nonzero = _{ ['1'..'9'] }
        digit = _{ ['0'..'9'] }
        integer = @{ ["0"] | (nonzero ~ digit*) }
        float = @{ integer+ ~ ["."] ~ integer+ }
        identifier = @{ !keyword ~ (alpha+ ~ digit*) }
        bool_literal = @{ ["true"] | ["false"] }
        literal_char = { whitespace | (!["\""] ~ any) }
        string_literal = @{ ["b\""] ~ literal_char* ~ ["\""] }
        block_start = { ["{"] }
        block_end = { ["}"] }
        lparen = { ["("] }
        rparen = { [")"] }
        annotation = { [":"] }
        arrow = { ["->"] }

        // operator
        op_or = { ["||"] }
        op_assign = { ["="] }
        op_and = { ["&&"] }
        op_eq = { ["=="] }
        op_ne = { ["!="] }
        op_le = { ["<="] }
        op_ge = { [">="] }
        op_gt = { ["<"] }
        op_lt = { [">"] }
        op_not = { ["!"] }

        // grammar define
        module = { decl_func* ~ statement* }
        decl_func = { ["fn"] ~ identifier ~  func_args ~ block }
        func_args = _{ lparen ~ (func_arg ~ [","])* ~ func_arg? ~ rparen }
        func_arg = _{ identifier ~ annotation ~ base_type }
        block = _{ block_start ~ statement* ~ block_end }
        statement = { var_decl | assignment | while_stmt | if_stmt | (expr ~ semi) | comment }
        var_decl = { let_decl_init }
        let_decl = { ["let"] ~ identifier }
        let_decl_init = { ["let"] ~ identifier ~  op_assign ~ (identifier | values) }
        assignment = { identifier ~ op_assign ~ (identifier | values)}
        values = { integer | float | string_literal }
        while_stmt = { ["while"] ~ expr ~ block }
        if_stmt = { ["if"] ~ expr ~ block ~ (["else"] ~ block)* }
        comment = _{ ["//"] ~ (!(["\r"] | ["\n"]) ~ any)* ~ (["\n"] | ["\r\n"] | ["\r"] | eoi) }
        op_infix = { op_or | op_and | op_eq | op_ne | op_le | op_ge | op_gt | op_lt }
        infix = { (identifier | func_call) ~ op_infix ~ (identifier | func_call) }
        prefix = { op_not? ~ (identifier | func_call) }
        expr = { infix | prefix }
        call_arg = { identifier | values }
        func_call = { identifier ~ lparen ~ (call_arg ~ [","])* ~ call_arg ~ rparen }

    } // endup grammer
   
    process! {

        module_ast(&self) -> Module {
            (stmts: _module()) => {
                Module {
                    body: stmts.into_iter().collect::<StmtBlock>(),
                }
            },
        }

        _module(&self) -> VecDeque<AstNode> {
            (_: statement, stmt: _statement(), mut tail: _module()) => {
                tail.push_front(stmt);
                tail
            },
            () => {
                let mut tail = VecDeque::new();
                tail.push_front(AstNode::Eof);
                tail
            },
        }

       // TODO: complete all statements
       _statement(&self) -> AstNode {
           (_: expr, stmt: _expr(), _: semi) => {
               stmt
           },
       }

        // TODO: infix
        _expr(&self) -> AstNode {
            (_: prefix, expr: _prefix()) => {
                expr
            },
        }

        _prefix(&self) -> AstNode {
            (&op_token: op_not, func: _func_call()) => {
                let optype = typeof_operator(op_token.to_string());
                AstNode::Prefix(optype, Box::new(func))
            },
            (&op_token: op_not, obj: _val_or_var()) => {
                let optype = typeof_operator(op_token.to_string());
                AstNode::Prefix(optype, Box::new(obj))
            },
        }

        _func_call(&self) -> AstNode {
            (_: func_call, method: _identifier(), args: _callargs()) => {
                AstNode::FnCall(Box::new(method), args.into_iter().collect::<Param>())
            },
        }

        _callargs(&self) -> VecDeque<AstNode> {
            (_: lparen, args: _feedargs()) => {
                args
            },
        }

        _feedargs(&self) -> VecDeque<AstNode> {
            (_: rparen) => {
                VecDeque::new()
            },
            (_: comma, arg: _val_or_var(), mut tail: _feedargs()) => {
                tail.push_front(arg);
                tail
            },
            (arg: _val_or_var(), mut tail: _feedargs()) => {
                tail.push_front(arg);
                tail
            },
        }

        _val_or_var(&self) -> AstNode {
            (ident: _identifier()) => {
                ident
            },
            (val: _values()) => {
                val
            },
        }

        _identifier(&self) -> AstNode {
            (&ident: identifier) => {
                AstNode::Identifer(ident.into(), 0)
            },
        }

        _values(&self) -> AstNode {
            (&val: integer) => {
                AstNode::Int(val.parse::<i32>().unwrap())
            },
            (&val: float) => {
                AstNode::Float(val.parse::<f32>().unwrap())
            },
            (&val: string_literal) => {
                AstNode::Str(val.to_string())
            }
        }
    } // endup process

} // endup impl_rdp
