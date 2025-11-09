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

fn apply_all<T, O, Accum, Project>(
    mut iter: impl Iterator<Item = O>,
    f: Accum,
    g: Project,
) -> Option<T>
where
    Accum: Fn(T, T) -> T,
    Project: Fn(O) -> T,
{
    let iter1 = iter.map(g);
    iter1.next().map(|first| iter1.fold(first, f))
}

// todo: use Result
pub fn eval(input: &Expr) -> &Expr {
    match input {
        Expr::Value(v) => v,
        Expr::Quoted(tag, expr) => panic!("Eval: Quoted: not implemented"),

        Expr::Container(tag, es) => {
            let res = es.iter().map(print).collect::<Vec<_>>().join(" ");
            match tag {
                ContainerKind::List => match es.first() {
                    Some(fun) => {
                        let (prim_idx, project, inject) = match eval(fun).as_string() {
                            "+" => (0, MalVal::as_i64, MalVal::number),
                            "-" => (1, MalVal::as_i64, MalVal::number),
                            "\\" => (2, MalVal::as_i64, MalVal::number),
                            "*" => (3, MalVal::as_i64, MalVal::number),
                            _ => panic!("Not Builtin!!"),
                        };
                        let prim_fn = OP_TABLE[prim_idx];
                        let args = &es[1..es.length()];
                        // the recursive call
                        let prim_args = args.iter().map(|a| eval(a)).collect::<Vec<_>>().join();
                        let payload = args.apply_all(prim_args, prim_fn, project);
                        // now rebox based on the types
                        inject(payload)
                    }
                    None => {
                        panic!("idk what here");
                    }
                },
                ContainerKind::Vec => panic!("Eval: Vec: not implemented"),
            }
        }

        Expr::HashMap(hmap) => panic!("Eval: HashMap: not implemented"),
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
}
