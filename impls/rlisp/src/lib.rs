use rustyline::{DefaultEditor, Result};

pub mod constants;
pub mod reader;
pub mod types;

use crate::types::*;
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

static OP_TABLE: [(fn(i64, i64) -> i64, fn(&Expr) -> i64, fn(i64) -> Expr); 4] = [
    (add, Expr::as_i64, Expr::number),
    (sub, Expr::as_i64, Expr::number),
    (mul, Expr::as_i64, Expr::number),
    (div, Expr::as_i64, Expr::number),
];

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("user> ")
}

fn apply_all<'a, T, Accum, Project, I>(iter: I, f: Accum, g: Project) -> MalResult<T>
where
    I: Iterator<Item = &'a MalResult<Expr>>,
    Accum: Fn(T, T) -> T,
    T: Copy,
    Project: Fn(&Expr) -> T,
{
    // Convert &Result<Expr> → Result<T>
    // This uses ? to stop early on the first error.
    let mut mapped = iter.map(|r| {
        let expr = r.as_ref()?; // go from &Result<Expr> to &Expr
        Ok::<T, MalError>(g(expr))
    });

    // Extract the first value (needed for fold)
    let first = mapped
        .next()
        .ok_or_else(|| MalError::other_error("apply_all: no args!"))??;

    // Fold the remaining values
    mapped.try_fold(first, |acc, t| {
        let v = t?;
        Ok::<T, MalError>(f(acc, v))
    })
}

pub fn eval_as_fun(
    expr: Expr,
) -> MalResult<(
    // TODO: make a better type for this
    fn(i64, i64) -> i64,
    for<'a> fn(&'a types::Expr) -> i64,
    fn(i64) -> types::Expr,
)> {
    // DESIGN: should evalAsFun know about OP_TABLE, or should eval?
    match expr {
        Expr::Value(MalVal::BSymbol(s)) => Ok(OP_TABLE[s as usize]),
        Expr::Value(MalVal::Symbol(s)) => panic!("{}: Not implemented yet!", s),
        other => MalError::not_afun_error(&format!("got: {}. Not a function", other)),
    }
}

pub fn eval(input: &Expr) -> MalResult<Expr> {
    #[cfg(debug_assertions)]
    println!("expr: {:?}", input);

    let result = match input {
        Expr::Quoted(_tag, _expr) => panic!("Eval: Quoted: not implemented"),

        Expr::Container(tag, es) => {
            match tag {
                ContainerKind::List => match es.first() {
                    Some(fun) => {
                        // START: change this to call evalAsFun and make that function
                        let sym = eval(fun)?;
                        let (prim_fn, project, inject) = eval_as_fun(sym)?;
                        let args = &es[1..es.len()];
                        // the recursive call
                        let prim_args = args.iter().map(|a| eval(a)).collect::<Vec<_>>();
                        let payload = apply_all(prim_args.iter(), prim_fn, project);

                        // now rebox based on the types
                        payload.map(inject)
                    }
                    None => {
                        // empty list
                        Ok(Expr::list(vec![]))
                    }
                },
                ContainerKind::Vec => panic!("Eval: Vec: not implemented"),
            }
        }
        Expr::HashMap(_hmap) => panic!("Eval: HashMap: not implemented"),
        val => Ok(val.clone()),
    };
    result
}

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Value(v) => match v {
            MalVal::Number(n) => format!("{}", n),
            MalVal::String(s) => format!("\"{}\"", s.clone()),
            MalVal::Symbol(s) => s.clone(),
            MalVal::BSymbol(s) => format!("{}", s),
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
