use types::MalType;

pub fn pr_str(m: &MalType) -> String {
    match *m {
        MalType::Int(x) => x.to_string(),
        MalType::Symbol(ref s) => s.clone(),
        MalType::Lambda(ref x, ref b) => {
            let mut s = "(λ".to_string();
            s.push_str(x);
            s.push('.');
            s.push_str(&pr_str(b));
            s.push(')');
            s
        }
        MalType::Apply(ref f, ref a) => {
            let mut s = "(".to_string();
            s.push_str(&pr_str(f));
            s.push(' ');
            s.push_str(&pr_str(a));
            s.push(')');
            s
        }
        MalType::Add(ref a, ref b) => {
            let mut s = "(".to_string();
            s.push_str(&pr_str(a));
            s.push('+');
            s.push_str(&pr_str(b));
            s.push(')');
            s
        }
        MalType::If(ref a, ref b, ref c) => {
            let mut s = "(if ".to_string();
            s.push_str(&pr_str(a));
            s.push(' ');
            s.push_str(&pr_str(b));
            s.push(' ');
            s.push_str(&pr_str(c));
            s.push(')');
            s
        }
        MalType::Error(ref e) => e.to_string(),
    }
}

#[test]
fn test_pr_str() {
    assert_eq!("123", pr_str(&MalType::Int(123)));
    assert_eq!("(1 2 3)",
               pr_str(&MalType::List(vec![MalType::Int(1), MalType::Int(2), MalType::Int(3)])));
}
