
extern crate llvm_sys as llvm;

use self::llvm::core::*;
use self::llvm::prelude::*;

use std::ptr;
use std::ffi::CString;
use crate::ast::*;

struct Generator {
    pub ctx: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,

}

impl Generator {
    pub unsafe fn new() -> Self {
        let _ctx = LLVMContextCreate();
        Generator {
            ctx: _ctx,
            module: LLVMModuleCreateWithName(b"__module\0".as_ptr() as *const _),
            builder: LLVMCreateBuilderInContext(_ctx),
        }
    }

    pub unsafe fn gen(&mut self, name: &String, module: &Vec<AstNode>) {
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

    fn gen_vardecl(&mut self, n: AstNode) {
        if let AstNode::VarDecl(ident, val, typ) = n {
        }
    }

    unsafe fn gen_fndecl(&mut self, n: AstNode) {
        if let AstNode::FnDecl(ident, param, block) = n {
            let function_name = ident_name(&ident);
            let function_type = unsafe {
                let return_type = self.typeof_valobj(ident_type(&ident.clone()));
                let mut param_types = self.gen_param_type(&param);
                LLVMFunctionType(return_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
            };
            let function = LLVMAddFunction(self.module, function_name.into_bytes().as_ptr() as *const _, function_type);
            let entry_name = CString::new("entry").unwrap();
            let block = LLVMAppendBasicBlockInContext(self.ctx, function, entry_name.as_ptr());
            LLVMPositionBuilderAtEnd(self.builder, block);

        }
    }

    unsafe fn gen_param_type(&mut self, n: &Vec<AstNode>) -> Vec<LLVMTypeRef> {
        let mut types = Vec::new();
        for ident in n {
            types.push(self.typeof_valobj(ident_type(ident)));
        }
        return types;
    }

    unsafe fn gen_block(&mut self, block: LLVMBasicBlockRef, n: AstNode) {
    }

    unsafe fn typeof_valobj(&mut self, t: AstType) -> LLVMTypeRef {
        match t {
            AstType::Int => LLVMInt64TypeInContext(self.ctx),
            AstType::Float => LLVMFloatTypeInContext(self.ctx),
            // TODO: AstType::Str => LLVMConstStringInContext(self.ctx),
            AstType::Bool => LLVMInt8TypeInContext(self.ctx),
            _ => LLVMInt8TypeInContext(self.ctx),
        }
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
            if a > 100 {
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
    semantic_check(stmts.clone());
    unsafe {

    let mut generator = Generator::new();
    generator.gen(&"demo".to_string(), &stmts);

    }
}
