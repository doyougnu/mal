use rustyline::{DefaultEditor, Result};

pub mod constants;
pub mod reader;
pub mod types;

use crate::types::{ContainerKind, Expr, MalVal, QuoteKind};
use constants::HISTORY;

fn add(l: i64, r: i64) -> i64 {
    l + r
}

fn sub(l: i64, r: i64) -> i64 {
    l - r
}

fn div(l: i64, r: i64) -> i64 {
    l / r
}

fn mul(l: i64, r: i64) -> i64 {
    l * r
}

static OP_TABLE: [fn(i64, i64) -> i64; 4] = [add, sub, div, mul];

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("user> ")
}

fn apply_all<'a, T, Accum, Project, I>(iter: I, f: Accum, g: Project) -> Option<T>
where
    I: Iterator<Item = &'a Expr>,
    Accum: Fn(T, T) -> T,
    Project: Fn(&Expr) -> T,
{
    let mut iter1 = iter.map(g);
    iter1.next().map(|first| iter1.fold(first, f))
}

// todo: use Result
pub fn eval(input: &Expr) -> Expr {
    println!("expr: {:?}", input);
    match input {
        Expr::Quoted(_tag, _expr) => panic!("Eval: Quoted: not implemented"),

        Expr::Container(tag, es) => {
            match tag {
                ContainerKind::List => match es.first() {
                    Some(fun) => {
                        let sym: &str = &eval(fun).to_string();
                        let (prim_idx, project, inject) = match sym {
                            "+" => (0, Expr::as_i64, Expr::number),
                            "-" => (1, Expr::as_i64, Expr::number),
                            "\\" => (2, Expr::as_i64, Expr::number),
                            "*" => (3, Expr::as_i64, Expr::number),
                            _ => panic!("Not Builtin!!"),
                        };
                        let prim_fn = OP_TABLE[prim_idx];
                        let args = &es[1..es.len()];
                        // the recursive call
                        let prim_args = args.iter().map(|a| eval(a)).collect::<Vec<_>>();
                        let payload = apply_all(prim_args.iter(), prim_fn, project);
                        // now rebox based on the types
                        match payload {
                            Some(result) => inject(result),
                            None => panic!("No result!"),
                        }
                    }
                    None => {
                        panic!("idk what here");
                    }
                },
                ContainerKind::Vec => panic!("Eval: Vec: not implemented"),
            }
        }
        Expr::HashMap(_hmap) => panic!("Eval: HashMap: not implemented"),
        val => val.clone(),
    }
}

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Value(v) => match v {
            MalVal::Number(n) => format!("{}", n),
            MalVal::String(s) => format!("\"{}\"", s.clone()),
            MalVal::Symbol(s) => s.clone(),
            MalVal::Keyword(k) => format!(":{}", k.clone()),
        },
        Expr::Quoted(tag, expr) => match tag {
            QuoteKind::Quote => format!("(quote {})", print(expr)),
            QuoteKind::Quasi => format!("(quasiquote {})", print(expr)),
            QuoteKind::Unquote => format!("(unquote {})", print(expr)),
            QuoteKind::SpliceUnquote => format!("(splice-unquote {})", print(expr)),
        },
        Expr::Container(tag, es) => {
            let res = es.iter().map(print).collect::<Vec<_>>().join(" ");
            match tag {
                ContainerKind::List => format!("({})", res),
                ContainerKind::Vec => format!("[{}]", res),
            }
        }
        Expr::HashMap(hmap) => {
            let res = hmap
                .iter()
                .map(|(k, v)| format!("{} {}", k, v))
                .collect::<Vec<_>>()
                .join(" ");
            format!("{{{}}}", res)
        }
    }
}
