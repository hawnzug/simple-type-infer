#[derive(Clone)]
pub enum MalType {
    Int(u32),
    Symbol(String),
    Lambda(String, Box<MalType>),
    Apply(Box<MalType>, Box<MalType>),
    Add(Box<MalType>, Box<MalType>),
    If(Box<MalType>, Box<MalType>, Box<MalType>),
    Error(String),
}
