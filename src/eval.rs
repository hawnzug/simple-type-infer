use types::Term;
use types::Typ;
use reader::read_str;
use printer::pr_str;
use std::collections::HashMap;

pub fn read(s: &str) -> Term {
    read_str(s)
}

pub fn print(m: Term) -> String {
    pr_str(&m)
}

pub fn rep(s: &str) -> String {
    print(eval(read(s)))
}

fn typesub(type_table: &IntMap, t: Typ) -> Typ {
    match t {
        Typ::TVar(i) => {
            if let Some(t) = type_table.get(&i) {
                return typesub(type_table, t.clone());
            } else {
                return Typ::TVar(i);
            }
        }
        Typ::Func(t1, t2) => {
            return Typ::Func(Box::new(typesub(type_table, *t1)),
                             Box::new(typesub(type_table, *t2)))
        }
        _ => return t,
    }
}

pub fn trep(s: &str) {
    let ast = read(s);
    println!("{}", pr_str(&ast));
    match teval(Vec::new(), ast, (0, HashMap::new())) {
        Ok((t, tev)) => {
            println!("{:?}", t);
            let (_, intmap) = tev;
            println!("{:?}", typesub(&intmap, t));
            println!("{:?}", intmap);
        }
        Err(err) => println!("{}", err),
    }
}

pub fn eval(ast: Term) -> Term {
    ast
}

type IntMap = HashMap<usize, Typ>;
type TVE = (usize, IntMap);
type TEnv = Vec<(String, Typ)>;
type EnvType = (Typ, TVE);

trait TEnvOp {
    fn lookup(&self, &str) -> Option<Typ>;
}

impl TEnvOp for TEnv {
    fn lookup(&self, s: &str) -> Option<Typ> {
        let len = self.len();
        for i in 0..len {
            let (ref name, ref t) = self[len - i - 1];
            if name == s {
                return Some(t.clone());
            }
        }
        return None;
    }
}

fn typechase(type_table: &IntMap, t: Typ) -> Typ {
    if let Typ::TVar(i) = t {
        if let Some(t) = type_table.get(&i) {
            return typechase(type_table, t.clone());
        }
    }
    return t;
}

fn unify(t1: Typ, t2: Typ, type_table: IntMap) -> Result<IntMap, String> {
    unify_help(typechase(&type_table, t1),
               typechase(&type_table, t2),
               type_table)
}

fn unify_help(t1: Typ, t2: Typ, type_table: IntMap) -> Result<IntMap, String> {
    match (t1, t2) {
        (Typ::TInt, Typ::TInt) => Ok(type_table),
        (Typ::TVar(v1), t2) => unifyv(v1, t2, type_table),
        (t1, Typ::TVar(v2)) => unifyv(v2, t1, type_table),
        (Typ::Func(t1a, t1r), Typ::Func(t2a, t2r)) => {
            match unify(*t1a, *t2a, type_table) {
                Ok(tab1) => unify(*t1r, *t2r, tab1),
                Err(err) => Err(err),
            }
        }
        _ => Err("constant mismatch".to_string()),
    }
}

fn unifyv(v1: usize, t2: Typ, mut type_table: IntMap) -> Result<IntMap, String> {
    if let Typ::TVar(v2) = t2 {
        if v1 == v2 {
            return Ok(type_table);
        } else {
            type_table.insert(v1, t2);
            return Ok(type_table);
        }
    } else {
        if occurs(v1, &t2, &type_table) {
            return Err("occurs check error".to_string());
        } else {
            type_table.insert(v1, t2);
            return Ok(type_table);
        }
    }
}

fn occurs(v: usize, t: &Typ, type_table: &IntMap) -> bool {
    match *t {
        Typ::TInt => false,
        Typ::Func(ref t1, ref t2) => occurs(v, t1, type_table) && occurs(v, t2, type_table),
        Typ::TVar(v2) => {
            if let Some(t) = type_table.get(&v) {
                occurs(v, t, type_table)
            } else {
                v == v2
            }
        }
    }
}

fn teval(mut env: TEnv, m: Term, tve0: TVE) -> Result<EnvType, String> {
    match m {
        Term::Int(_) => Ok((Typ::TInt, tve0)),
        Term::Symbol(s) => {
            match env.lookup(&s) {
                Some(t) => Ok((t, tve0)),
                _ => Err("Cannot find symbol".to_string()),
            }
        }
        Term::Lambda(symb, body) => {
            let (tv, intmap) = tve0;
            env.push((symb, Typ::TVar(tv)));
            match teval(env, *body, (tv + 1, intmap)) {
                Ok((te, tve1)) => Ok((Typ::Func(Box::new(Typ::TVar(tv)), Box::new(te)), tve1)),
                Err(err) => Err(err),
            }
        }
        Term::Apply(m1, m2) => {
            match teval(env.clone(), *m1, tve0) {
                Ok((t1, tve1)) => {
                    match teval(env, *m2, tve1) {
                        Ok((t2, tve2)) => {
                            let (tv, intmap) = tve2;
                            match unify(t1,
                                        Typ::Func(Box::new(t2), Box::new(Typ::TVar(tv))),
                                        intmap) {
                                Ok(im) => Ok((Typ::TVar(tv), (tv + 1, im))),
                                Err(e) => Err(e),
                            }
                        }
                        e@Err(_) => e,
                    }
                }
                e@Err(_) => e,
            }
        }
        Term::Add(m1, m2) => {
            match teval(env.clone(), *m1, tve0) {
                Ok((t1, tve1)) => {
                    match teval(env, *m2, tve1) {
                        Ok((t2, tve2)) => {
                            let (tv, intmap) = tve2;
                            match unify(t1, Typ::TInt, intmap) {
                                Ok(im) => {
                                    match unify(t2, Typ::TInt, im) {
                                        Ok(imm) => Ok((Typ::TInt, (tv, imm))),
                                        Err(e) => Err(e + "\nTry to add non-integers"),
                                    }
                                }
                                Err(e) => Err(e + "\nTry to add non-integers"),
                            }
                        }
                        e@Err(_) => e,
                    }
                }
                e@Err(_) => e,
            }
        }
        Term::If(m1, m2, m3) => {
            match teval(env.clone(), *m1, tve0) {
                Ok((t1, tve1)) => {
                    match teval(env.clone(), *m2, tve1) {
                        Ok((t2, tve2)) => {
                            match teval(env, *m3, tve2) {
                                Ok((t3, tve3)) => {
                                    let (tv, intmap) = tve3;
                                    match unify(t1, Typ::TInt, intmap) {
                                        Ok(im) => {
                                            match unify(t2.clone(), t3, im) {
                                                Ok(imm) => Ok((t2, (tv, imm))),
                                                Err(e) => {
                                                    Err(e + "\nBranches of if have different types")
                                                }
                                            }
                                        }
                                        Err(e) => Err(e + "\nTry to compare a non-integer to 0"),
                                    }
                                }
                                e@Err(_) => e,
                            }
                        }
                        e@Err(_) => e,
                    }
                }
                e@Err(_) => e,
            }
        }
        _ => Ok((Typ::TInt, tve0)),
    }
}

#[test]
fn test_lookup() {
    let env = vec![("a".to_string(), Typ::TInt),
                   ("x".to_string(), Typ::TInt),
                   ("y".to_string(), Typ::TInt),
                   ("z".to_string(), Typ::TInt),
                   ("x".to_string(), Typ::TVar(0))];
    assert_eq!(env.lookup("x"), Some(Typ::TVar(0)));
    assert_eq!(env.lookup("y"), Some(Typ::TInt));
    assert_eq!(env.lookup("z"), Some(Typ::TInt));
    assert_eq!(env.lookup("a"), Some(Typ::TInt));
}
