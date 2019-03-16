
use std::collections::HashMap;
use crate::ast::AstType;
use std::fmt;

type DefTable = HashMap<String, AstType>;
type Scope = Vec<DefTable>;

#[derive(Debug)]
pub struct Env {
    pub global: DefTable,
    pub local: Scope,
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
            local: Vec::new(),
        }
    }

    pub fn global_def(&mut self, var: &String, t: AstType) {
        self.global.insert(var.clone(), t);
    }

    pub fn global_has(&mut self, var: &String) -> bool {
        self.global.contains_key(var)
    }

    pub fn global_resolve(&mut self, var: &String) -> Option<&AstType> {
        self.global.get(var)
    }

    pub fn enter_scope(&mut self) {
        self.local.push(HashMap::new());
    }

    pub fn local_def(&mut self, var: &String, t: AstType) {
        if let Some(mut top) = self.local.pop() {
            top.insert(var.to_string(), t);
            self.local.push(top);
        }
    }

    pub fn local_has(&mut self, var: &String) -> bool {
        let mut ok = false;
        if let Some(top) = self.local.pop() {
            ok = top.contains_key(var);
            self.local.push(top);
        }
        ok
    }

    pub fn leave_scope(&mut self) {
        self.local.pop();
    }

    pub fn resolve(&self, var: &String) -> Option<AstType> {
        let mut stk = self.local.clone(); stk.reverse();
        for s in stk.iter() {
            if s.contains_key(var) {
                return s.get(var).cloned();
            }
        }
        self.global.get(var).cloned()
    }

    pub fn update(&mut self, var: &String, typ: AstType) {
        let len = self.local.len();
        for idx in 0..len {
            if self.local[len-idx-1].contains_key(var) {
                self.local[len-idx-1].entry(var.clone()).and_modify(|e| { *e = typ.clone() });
                return ;
            }
        }
        self.global.entry(var.clone()).and_modify(|e| { *e = typ.clone() });
    }

    pub fn can_resolve(&self, var: &String) -> bool {
        match self.resolve(var) {
            Some(e) => true,
            _ => false,
        }
    }

}
