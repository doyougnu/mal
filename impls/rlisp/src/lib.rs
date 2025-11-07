use rustyline::{DefaultEditor, Result};

pub mod constants;
pub mod reader;
pub mod types;

use crate::types::{ContainerKind, Expr, MalVal, QuoteKind};
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
