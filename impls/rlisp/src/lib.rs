use rustyline::{DefaultEditor, Result};

pub mod constants;
pub mod env;
pub mod reader;
pub mod types;

use crate::{env::ReplEnv, types::*};
use constants::HISTORY;

fn add(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li + lr))
}

fn sub(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li - lr))
}

fn div(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li / lr))
}

fn mul(l: Expr, r: Expr) -> MalResult<Expr> {
    let li = l.as_i64()?;
    let lr = r.as_i64()?;
    Ok(Expr::number(li * lr))
}

static OP_TABLE: [MalVal; 4] = [
    MalVal::Lambda(Fun::Binary(add)),
    MalVal::Lambda(Fun::Binary(sub)),
    MalVal::Lambda(Fun::Binary(mul)),
    MalVal::Lambda(Fun::Binary(div)),
];

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("user> ")
}

fn apply(mut args: Vec<Expr>, b: Fun) -> MalResult<Expr> {
    // NOTE to self: notice the mut args. thats only so I can call remove on the
    // vec. This is needed because I want to own the data of the vec for
    // processing. Without the mut I would only be able to borrow a value with a
    // reference
    match b {
        Fun::Unary(f) => f(args.remove(0)),
        Fun::Binary(f) => {
            let first = args.remove(0);
            args.into_iter().try_fold(first, |acc, x| f(acc, x))
        }
        Fun::Nary(f) => f(&args[..]),
    }
}

pub fn eval_as_fun(env: &ReplEnv, expr: Expr) -> MalResult<Fun> {
    // DESIGN: should evalAsFun know about OP_TABLE, or should eval?
    match expr {
        // TODO: builtins should pre-populate the environment
        Expr::Value(MalVal::BSymbol(s)) => Ok(OP_TABLE[s as usize].clone()),
        Expr::Value(MalVal::Symbol(s)) => env.get(&s),
        other => MalError::not_afun_error(&format!("got: {}. Not a function", other)),
    }
}

pub fn eval_as_val(env: &ReplEnv, expr: Expr) -> MalResult<MalVal> {
    match eval(env, expr)? {
        Expr::Value(v) => Ok(v),
        other => MalError::other_error(&format!("got: {}. Expected value", other)),
    }
}

pub fn eval(env: &ReplEnv, input: Expr) -> MalResult<Expr> {
    #[cfg(debug_assertions)]
    println!("expr: {:?}", input);

    let result = match input {
        Expr::Quoted(_tag, _expr) => panic!("Eval: Quoted: not implemented"),

        Expr::Container(cont) => {
            match cont {
                MalContainer::List(mut es) => {
                    println!("ES: {:?}", es);
                    if es.len() >= 2 {
                        let fun = es.remove(0);
                        let sym = eval(env, *fun)?;
                        let prim_fn = eval_as_fun(env, sym)?;

                        // the recursive call
                        let prim_args: Vec<MalResult<Expr>> =
                            es.into_iter().map(|a| eval(env, *a)).collect::<Vec<_>>();
                        let prim_args: MalResult<Vec<Expr>> = prim_args.into_iter().collect();
                        prim_args.and_then(|p| apply(p, prim_fn))
                    } else if es.len() == 1 {
                        // a value
                        eval(env, *es.remove(0))
                    } else {
                        // an empty list
                        Ok(Expr::list(vec![]))
                    }
                }
                MalContainer::Vector(_) => panic!("Vec not implemented"),
                MalContainer::HashMap(_hmap) => panic!("Eval: HashMap: not implemented"),
            }
        }
        // only other case are literals
        val => {
            println!("Val: {:?}", val);
            Ok(val)
        }
    };
    println!("result: {:?}", result);
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
        other => format!("{}", other),
    }
}
