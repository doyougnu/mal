use rustyline::{DefaultEditor, Result};

pub mod constants;
pub mod reader;
pub mod types;

use crate::types::{Expr, MalVal, QuoteKind};
use constants::HISTORY;

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("user> ")
}

pub fn eval(input: &Expr) -> &Expr {
    input
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
        Expr::List(exprs) => {
            let result = exprs.iter().map(|e| print(e)).collect::<Vec<_>>().join(" ");
            format!("({})", result)
        }
    }
}
