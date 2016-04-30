use chomp::*;
use chomp::ascii::is_alpha;
use chomp::ascii::is_digit;
use chomp::ascii::skip_whitespace;
use std::str::from_utf8;
use types::MalType;

pub fn read_str(s: &str) -> MalType {
    match parse_only(parse_all, s.as_bytes()) {
        Ok(e@MalType::Error(_)) => e,
        Ok(m) => m,
        _ => MalType::Error("Syntax Error".to_string()),
    }
}

fn parse_int(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        let later = take_while1(is_digit);
        ret to_malint(from_utf8(later).unwrap().to_string())
    }
}

fn parse_symbol(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        let s = take_while1(is_alpha);
        ret MalType::Symbol(from_utf8(s).unwrap().to_string())
    }
}

fn parse_lambda(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        string(b"lambda");
        take_while1(|c| c == b' ');
        let arg = take_while1(is_alpha);
        let body = parse_all();
        token(b')');
        ret MalType::Lambda(from_utf8(arg).unwrap().to_string(), Box::new(body))
    }
}

fn parse_add(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        token(b'+');
        take_while1(|c| c == b' ');
        let a = parse_all();
        let b = parse_all();
        token(b')');
        ret MalType::Add(Box::new(a), Box::new(b))
    }
}

fn parse_if(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        string(b"if");
        take_while1(|c| c == b' ');
        let a = parse_all();
        let b = parse_all();
        let c = parse_all();
        token(b')');
        ret MalType::If(Box::new(a), Box::new(b), Box::new(c))
    }
}

fn parse_apply(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        token(b'(');
        let func = parse_all();
        let arg = parse_all();
        token(b')');
        ret MalType::Apply(Box::new(func), Box::new(arg))
    }
}

fn parse_all(i: Input<u8>) -> U8Result<MalType> {
    let r = parser!{
        parse_int() <|>
        parse_symbol() <|>
        parse_lambda() <|>
        parse_apply() <|>
        parse_if() <|>
        parse_add()
    };
    parse!{i;
        skip_whitespace();
        let t = r();
        skip_whitespace();
        ret t
    }
}

fn to_malint(c: String) -> MalType {
    match c.parse::<u32>() {
        Ok(x) => MalType::Int(x),
        _ => MalType::Error("Number overflow".to_string()),
    }
}
