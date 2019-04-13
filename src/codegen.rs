
extern crate llvm_sys as llvm;

use self::llvm::core::*;
use self::llvm::prelude::*;

use llvm::LLVMIntPredicate::*;
use llvm::LLVMRealPredicate::*;
use std::ptr;
use std::ffi::CString;
use crate::ast::*;
use std::collections::HashMap;



type SymbolTable = HashMap<String, LLVMValueRef>;

struct LLVMGenerator {
    pub ctx: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,

    pub functions: SymbolTable,
    pub global: SymbolTable,
    pub locals: Vec<SymbolTable>,
}

/// Convert a string literal into a C string.
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

impl LLVMGenerator {
    pub unsafe fn new() -> Self {
        let _ctx = LLVMContextCreate();
        LLVMGenerator {
            ctx: _ctx,
            module: LLVMModuleCreateWithName(b"__module\0".as_ptr() as *const _),
            builder: LLVMCreateBuilderInContext(_ctx),
            functions: HashMap::new(),
            global: HashMap::new(),
            locals: Vec::new(),
        }
    }

    pub unsafe fn run(&mut self, name: &String, module: &Vec<AstNode>) {
        for item in module {
            match item {
                AstNode::FnDecl(_, _, _) => self.gen_fndecl(item.clone()),
                _ => (),
            }
        }

        let ir = LLVMPrintModuleToString(self.module);
        let ir_cstring = CString::from_raw(ir);
        let ir = ir_cstring.to_str().unwrap();
        println!("{}", ir);

        // let out_file = CString::new("llvm-ir.ll").unwrap();
        // LLVMPrintModuleToFile(self.module, out_file.as_ptr(), ptr::null_mut());
        LLVMDisposeBuilder(self.builder);
        LLVMDisposeModule(self.module);
        LLVMContextDispose(self.ctx);
    }

    fn enter_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    fn leave_scope(&mut self) {
        self.locals.pop();
    }

    fn get(&self, var: &String) -> Option<LLVMValueRef> {
        let mut stk = self.locals.clone(); stk.reverse();
        for s in stk.iter() {
            if s.contains_key(var) {
                return s.get(var).cloned();
            }
        }
        self.global.get(var).cloned()
    }

    unsafe fn gen_fndecl(&mut self, n: AstNode) {
        if let AstNode::FnDecl(ident, param, block) = n {
            let function_name = ident_name(&ident);
            let function_type = unsafe {
                let return_type = self.typeof_llvm(ident_type(&ident.clone()));
                let mut param_types = self.gen_param_type(&param);
                LLVMFunctionType(return_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
            };
            let function = LLVMAddFunction(self.module, function_name.into_bytes().as_ptr() as *const _, function_type);
            let entry = CString::new("entry").unwrap();
            self.functions.insert(ident_name(&ident), function);
            self.enter_scope();
            let bb = LLVMAppendBasicBlockInContext(self.ctx, function, entry.as_ptr());
            LLVMPositionBuilderAtEnd(self.builder, bb);
            self.alloc_param(function, &param);
            self.gen_block(bb, &block);
            self.leave_scope();
        }
    }

    unsafe fn alloc_param(&mut self, Fn: LLVMValueRef, p: &Vec<AstNode>) {
        for (idx, var) in p.iter().enumerate() {
            let cname = CString::new(ident_name(&var)).unwrap();
            let ty = self.typeof_llvm(ident_type(&var));
            let _var = LLVMBuildAlloca(self.builder, ty, cname.as_ptr());
            self.push_var(ident_name(&var), _var);
            let val = LLVMGetParam(Fn, idx as u32);
            LLVMBuildStore(self.builder, _var, val);
        }
    }

    fn push_var(&mut self, var: String, val: LLVMValueRef) {
        let idx = self.locals.len();
        self.locals[idx-1].insert(var, val);
    }

    unsafe fn gen_vardecl(&mut self, var: &AstNode) {
        if let AstNode::VarDecl(ident, val, _) = var {
            let cname = CString::new(ident_name(&ident)).unwrap();
            let ty = self.typeof_llvm(ident_type(&ident));
            let _var = LLVMBuildAlloca(self.builder, ty, cname.as_ptr());
            self.push_var(ident_name(&ident), _var);
            if !nil_node(val) {
                let val = self.gen_initializer(val).unwrap();
                LLVMBuildStore(self.builder, _var, val);
            }
        }
    }

    unsafe fn gen_initializer(&mut self, expr: &AstNode) -> Option<LLVMValueRef> {
        match expr {
            AstNode::BinaryOp(_, _, _, _) => self.gen_numberical(expr),
            AstNode::Int(v) => Some(LLVMConstInt(self.i64_type(), *v as u64, 1)),
            AstNode::Float(v) => Some(LLVMConstReal(self.f64_type(), *v as f64)),
            _ => unreachable!(),
        }
    }

    unsafe fn gen_value(&mut self, val: &AstNode) -> LLVMValueRef {
        match val {
            AstNode::Int(v) => LLVMConstInt(self.i64_type(), *v as u64, 1),
            AstNode::Float(v) => LLVMConstReal(self.f64_type(), *v as f64),
            AstNode::FnCall(ident, args) => self.gen_call(val),
            AstNode::Ident(name, _) => self.get(name).unwrap(),
            // TODO: supports String
            _ => unreachable!(),
        }
    }

    unsafe fn gen_call(&mut self, Fn: &AstNode) -> LLVMValueRef {
        if let AstNode::FnCall(ident, args) = Fn {
                let name = ident_name(&ident);
                let fnptr = self.functions[&name];
                let mut _args = vec!();
                for item in args { _args.push(self.gen_value(item)); }
                return LLVMBuildCall(self.builder, fnptr, _args.as_mut_ptr(), _args.len() as u32, c_str!(""))
        }
        unreachable!();
    }

    unsafe fn gen_expr_value(&mut self, expr: &AstNode) -> LLVMValueRef {
        match expr {
            AstNode::BinaryOp(lhs, op, rhs, ty) => {
                match *lhs.clone() {
                    AstNode::BinaryOp(_, _, _, _) => self.gen_expr_value(lhs),
                    _ => self.gen_expr_cmp(expr),
                }
            },
            _ => self.gen_value(expr),
        }
    }

    fn llvm_int_op(&mut self, op: &Operator) -> llvm::LLVMIntPredicate {
        match op {
            Operator::OpEq => LLVMIntEQ,
            Operator::OpGt => LLVMIntSGT,
            _ => unreachable!(),
        }
    }

    fn llvm_float_op(&mut self, op: &Operator) -> llvm::LLVMRealPredicate {
        match op {
            Operator::OpEq => LLVMRealOEQ,
            Operator::OpGt => LLVMRealOGT,
            _ => unreachable!(),
        }
    }

    unsafe fn gen_expr_cmp(&mut self, expr: &AstNode) -> LLVMValueRef {
        if let AstNode::BinaryOp(lhs, op, rhs, ty) = expr {
            let lval = self.gen_value(lhs);
            let rval = self.gen_value(rhs);
            let val = match ty {
                AstType::Float => LLVMBuildFCmp(self.builder, self.llvm_float_op(op), lval, rval, c_str!("")),
                AstType::Int => LLVMBuildICmp(self.builder, self.llvm_int_op(op), lval, rval, c_str!("")),
                _ => unreachable!(),
            };
            return val;
        }
        unreachable!();
    }

    unsafe fn gen_numberical(&mut self, expr: &AstNode) -> Option<LLVMValueRef> {
        if let AstNode::BinaryOp(var, op, val, _) = expr {
            let retval = match *var.clone() {
                AstNode::BinaryOp(_, _, _, _) => self.gen_numberical(&var.clone()),
                AstNode::Ident(name, ty) => {
                    let val = self.gen_value(val);
                    let _var = self.get(&name).unwrap();
                    match op {
                        Operator::OpPlus => { Some(LLVMBuildAdd(self.builder, _var, val, c_str!(""))) }
                        _ => None,
                    }
                },
                _ => None,
            };
            return retval;
        }
        unreachable!();
    }

    unsafe fn gen_param_type(&mut self, n: &Vec<AstNode>) -> Vec<LLVMTypeRef> {
        let mut ty = Vec::new();
        for ident in n { ty.push(self.typeof_llvm(ident_type(ident))); }
        return ty;
    }

    unsafe fn gen_block(&mut self, block: LLVMBasicBlockRef, stmts: &Vec<AstNode>) {
        for stmt in stmts {
            match stmt {
                AstNode::VarDecl(_, _, _) => self.gen_vardecl(stmt),
                AstNode::IfStmt(_, _, _) => self.gen_ifstmt(stmt),
                _ => (),
            }
        }
    }

    unsafe fn gen_ifstmt(&mut self, stmt: &AstNode) {
        if let AstNode::IfStmt(cond, Tstmt, Fstmt) = stmt {
            self.gen_expr_value(cond);
        }
    }

    unsafe fn typeof_llvm(&mut self, t: AstType) -> LLVMTypeRef {
        match t {
            AstType::Int => LLVMInt64TypeInContext(self.ctx),
            AstType::Float => LLVMFloatTypeInContext(self.ctx),
            // TODO: AstType::Str => LLVMConstStringInContext(self.ctx),
            AstType::Bool => LLVMInt8TypeInContext(self.ctx),
            _ => LLVMInt8TypeInContext(self.ctx),
        }
    }

    unsafe fn i64_type(&self) -> LLVMTypeRef {
        LLVMInt64TypeInContext(self.ctx)
    }

    unsafe fn f64_type(&self) -> LLVMTypeRef {
        LLVMFloatTypeInContext(self.ctx)
    }

    unsafe fn bool_type(&self) -> LLVMTypeRef {
        LLVMInt8TypeInContext(self.ctx)
    }

}

#[test]
fn codegen_test() {
    use crate::ast::*;
    use crate::semantic::*;
    use crate::codegen::*;
    use crate::grammar::ModuleParser;
    let sources = r#"
        fn foo1(a: int, b: int) -> int {
            let c = a + 1001;
            let d: int;
            let ok = 123.456;
            if ok > 100.123 {
                d = b + 1000 + c + a;
            }
            a
        }

        fn foo2(a: int) -> bool {
            a == 100
        }

        fn fact(n: int) -> int {
            if n == 1 { return 1; }
            else { return fact(n - 1) * n; }
        }

        let a = 1000 + 10;
        fn main() {
            let b: int;
            a = foo1(a, 1001) + 123 + foo1(a, 100+101);
            b = foo1(123, a);
            while a > b + 100 {
                b = a + foo1(a, b);
            }
        }
    "#;
    println!("{}", sources);
    let stmts = ModuleParser::new().parse(sources).unwrap();
    let typed_ast = semantic_check(stmts);
    unsafe {

    let mut generator = LLVMGenerator::new();
    generator.run(&"demo".to_string(), &typed_ast);

    }
}
