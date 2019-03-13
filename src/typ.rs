
#[derive(Debug, Clone)]
pub enum AstType {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    // TODO: extend type: struct, enum, interface ...
    Ext(String),
    Undef,
}
