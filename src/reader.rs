use chomp::*;
use chomp::ascii::is_alpha;
use chomp::ascii::is_digit;
use chomp::ascii::skip_whitespace;
use std::str::from_utf8;
use types::Term;

pub fn read_str(s: &str) -> Term {
    match parse_only(parse_all, s.as_bytes()) {
        Ok(e@Term::Error(_)) => e,
        Ok(m) => m,
        _ => Term::Error("Syntax Error".to_string()),
    }
}

fn parse_int(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        let later = take_while1(is_digit);
        ret to_malint(from_utf8(later).unwrap().to_string())
    }
}

fn parse_symbol(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        let s = take_while1(is_alpha);
        ret Term::Symbol(from_utf8(s).unwrap().to_string())
    }
}

fn parse_lambda(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        string(b"lambda");
        take_while1(|c| c == b' ');
        let arg = take_while1(is_alpha);
        let body = parse_all();
        token(b')');
        ret Term::Lambda(from_utf8(arg).unwrap().to_string(), Box::new(body))
    }
}

fn parse_add(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        token(b'+');
        take_while1(|c| c == b' ');
        let a = parse_all();
        let b = parse_all();
        token(b')');
        ret Term::Add(Box::new(a), Box::new(b))
    }
}

fn parse_if(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        token(b'(');
        skip_whitespace();
        string(b"if");
        take_while1(|c| c == b' ');
        let a = parse_all();
        let b = parse_all();
        let c = parse_all();
        token(b')');
        ret Term::If(Box::new(a), Box::new(b), Box::new(c))
    }
}

fn parse_apply(i: Input<u8>) -> U8Result<Term> {
    parse!{i;
        token(b'(');
        let func = parse_all();
        let arg = parse_all();
        token(b')');
        ret Term::Apply(Box::new(func), Box::new(arg))
    }
}

fn parse_all(i: Input<u8>) -> U8Result<Term> {
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

fn to_malint(c: String) -> Term {
    match c.parse::<u32>() {
        Ok(x) => Term::Int(x),
        _ => Term::Error("Number overflow".to_string()),
    }
}
