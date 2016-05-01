use types::Term;
use types::Typ;

pub fn pr_str(m: &Term) -> String {
    match *m {
        Term::Int(x) => x.to_string(),
        Term::Symbol(ref s) => s.clone(),
        Term::Lambda(ref x, ref b) => {
            let mut s = "(λ".to_string();
            s.push_str(x);
            s.push('.');
            s.push_str(&pr_str(b));
            s.push(')');
            s
        }
        Term::Apply(ref f, ref a) => {
            let mut s = "(".to_string();
            s.push_str(&pr_str(f));
            s.push(' ');
            s.push_str(&pr_str(a));
            s.push(')');
            s
        }
        Term::Add(ref a, ref b) => {
            let mut s = "(".to_string();
            s.push_str(&pr_str(a));
            s.push('+');
            s.push_str(&pr_str(b));
            s.push(')');
            s
        }
        Term::If(ref a, ref b, ref c) => {
            let mut s = "(if ".to_string();
            s.push_str(&pr_str(a));
            s.push(' ');
            s.push_str(&pr_str(b));
            s.push(' ');
            s.push_str(&pr_str(c));
            s.push(')');
            s
        }
        Term::Error(ref e) => e.to_string(),
    }
}

pub fn pr_type(m: &Typ) -> String {
    match m.clone() {
        Typ::TInt => "int".to_string(),
        Typ::TVar(x) => "t".to_string() + &x.to_string(),
        Typ::Func(t1, t2) => "(".to_string() + &pr_type(&*t1) + " -> " + &pr_type(&*t2) + ")",
    }
}
