
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

    pub unsafe fn gen(&mut self, name: &String, module: Vec<AstNode>) {
        for item in module {
            match item {
                _ => ()
            }
        }

        let out_file = CString::new("output.ll").unwrap();
        llvm::core::LLVMPrintModuleToFile(self.module, out_file.as_ptr(), ptr::null_mut());
        llvm::core::LLVMDisposeBuilder(self.builder);
        llvm::core::LLVMDisposeModule(self.module);
        llvm::core::LLVMContextDispose(self.ctx);
    }

    fn gen_vardecl(&mut self, n: AstNode) {
        if let AstNode::VarDecl(ident, val, typ) = n {
        }
    }
}


