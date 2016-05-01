use std::fmt;
use printer::pr_type;

#[derive(Clone)]
pub enum Term {
    Int(u32),
    Symbol(String),
    Lambda(String, Box<Term>),
    Apply(Box<Term>, Box<Term>),
    Add(Box<Term>, Box<Term>),
    If(Box<Term>, Box<Term>, Box<Term>),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub enum Typ {
    TInt,
    Func(Box<Typ>, Box<Typ>),
    TVar(usize),
}

impl fmt::Debug for Typ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", pr_type(self))
    }
}
