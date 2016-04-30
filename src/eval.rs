use types::MalType;
use reader::read_str;
use printer::pr_str;

pub fn read(s: &str) -> MalType {
    read_str(s)
}

pub fn print(m: MalType) -> String {
    pr_str(&m)
}

pub fn rep(s: &str) -> String {
    print(eval(read(s)))
}

pub fn eval(ast: MalType) -> MalType {
    ast
}
