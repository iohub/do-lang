
use std::collections::HashMap;
use crate::ast::AstType;
use crate::ast::AstNode;
use crate::ast::ident_name;
use std::fmt;

type DefTable = HashMap<String, AstType>;
type Scope = Vec<DefTable>;

#[derive(Debug)]
pub struct Env {
    pub global: DefTable,
    pub locals: Scope,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Env {
    pub fn new() -> Self {
        Env {
            global: HashMap::new(),
            locals: Vec::new(),
        }
    }

    pub fn global_def(&mut self, var: &String, t: AstType) {
        self.global.insert(var.clone(), t);
    }

    pub fn global_defined(&mut self, var: &String) -> bool {
        self.global.contains_key(var)
    }

    pub fn global_resolve(&mut self, var: &String) -> Option<&AstType> {
        self.global.get(var)
    }

    pub fn enter_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    pub fn local_def(&mut self, var: &String, t: AstType) {
        if let Some(mut top) = self.locals.pop() {
            top.insert(var.to_string(), t);
            self.locals.push(top);
        }
    }

    pub fn _local_defined(&mut self, var: &String) -> bool {
        let mut ok = false;
        if let Some(top) = self.locals.pop() {
            ok = top.contains_key(var);
            self.locals.push(top);
        }
        ok
    }

    pub fn leave_scope(&mut self) {
        self.locals.pop();
    }

    pub fn lookup(&self, var: &String) -> Option<AstType> {
        let mut stk = self.locals.clone(); stk.reverse();
        for s in stk.iter() {
            if s.contains_key(var) {
                return s.get(var).cloned();
            }
        }
        self.global.get(var).cloned()
    }

    pub fn update(&mut self, var: &mut AstNode, typ: AstType) {
        let len = self.locals.len();
        let name = ident_name(var);
        if let AstNode::Ident(_, ref mut vtyp) = var {
            *vtyp = typ.clone();
        }
        for idx in 0..len {
            if self.locals[len-idx-1].contains_key(&name) {
                self.locals[len-idx-1].entry(name).and_modify(|e| { *e = typ });
                return ;
            }
        }
        self.global.entry(name).and_modify(|e| { *e = typ });
    }

    pub fn can_lookup(&self, var: &String) -> bool {
        match self.lookup(var) {
            Some(_) => true,
            _ => false,
        }
    }

}
