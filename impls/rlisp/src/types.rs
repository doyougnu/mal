use nom::lib::std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MalVal {
    Number(i64),
    Symbol(String),
    String(String),
    Keyword(String),
}

impl MalVal {
    pub fn number(n: i64) -> Self {
        MalVal::Number(n)
    }

    pub fn symbol(s: String) -> Self {
        MalVal::Symbol(s)
    }

    pub fn string(s: String) -> Self {
        MalVal::String(s)
    }

    pub fn keyword(s: String) -> Self {
        MalVal::Keyword(s)
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            MalVal::Number(i) => *i,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            MalVal::Symbol(i) => *i,
            MalVal::String(i) => *i,
            MalVal::Keyword(i) => *i,
        }
    }
}

impl fmt::Display for MalVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalVal::Number(n) => write!(f, "{}", n),
            MalVal::Symbol(s) => write!(f, "{}", s),
            MalVal::String(s) => write!(f, "\"{}\"", s),
            MalVal::Keyword(k) => write!(f, ":{}", k),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteKind {
    Quote,
    Unquote,
    Quasi,
    SpliceUnquote,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerKind {
    List,
    Vec,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(MalVal),
    Container(ContainerKind, Vec<Expr>),
    HashMap(HashMap<MalVal, Box<Expr>>),
    Quoted(QuoteKind, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Value(v) => write!(f, "{}", v),
            Expr::Container(k, e) => {
                let ls = e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                match k {
                    ContainerKind::List => write!(f, "({})", ls),
                    ContainerKind::Vec => write!(f, "[{}]", ls),
                }
            }
            Expr::HashMap(hmap) => {
                let smap = hmap
                    .iter()
                    .map(|(k, v)| format!("{} {}", k, v))
                    .collect::<Vec<_>>()
                    .join(" ");

                write!(f, "{{{}}}", smap)
            }
            Expr::Quoted(k, e) => match k {
                QuoteKind::Quote => write!(f, "(quote {})", e),
                QuoteKind::Unquote => write!(f, "(unquote {})", e),
                QuoteKind::Quasi => write!(f, "(quasiquote {})", e),
                QuoteKind::SpliceUnquote => write!(f, "(splice-unquote {})", e),
            },
        }
    }
}

impl Expr {
    pub fn number(n: i64) -> Self {
        Expr::Value(MalVal::number(n))
    }

    pub fn symbol(s: String) -> Self {
        Expr::Value(MalVal::symbol(s))
    }

    pub fn string(s: String) -> Self {
        Expr::Value(MalVal::string(s))
    }

    pub fn keyword(s: String) -> Self {
        Expr::Value(MalVal::keyword(s))
    }

    pub fn list(es: Vec<Expr>) -> Self {
        Expr::Container(ContainerKind::List, es)
    }

    pub fn vector(es: Vec<Expr>) -> Self {
        Expr::Container(ContainerKind::Vec, es)
    }

    pub fn hash_map(es: HashMap<MalVal, Box<Expr>>) -> Self {
        Expr::HashMap(es)
    }

    pub fn quote(expr: Expr) -> Self {
        Expr::Quoted(QuoteKind::Quote, Box::new(expr))
    }

    pub fn quasi(expr: Expr) -> Self {
        Expr::Quoted(QuoteKind::Quasi, Box::new(expr))
    }

    pub fn unquote(expr: Expr) -> Self {
        Expr::Quoted(QuoteKind::Unquote, Box::new(expr))
    }

    pub fn splice_unquote(expr: Expr) -> Self {
        Expr::Quoted(QuoteKind::SpliceUnquote, Box::new(expr))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInOp {
    Add,
    Sub,
    Mul,
    Div,
}
