use nom::error::{ErrorKind, FromExternalError, ParseError};
use nom::lib::std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MalVal {
    Number(i64),
    Symbol(String),
    String(String),
    Keyword(String),
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
        Expr::Value(MalVal::Number(n))
    }

    pub fn symbol(s: String) -> Self {
        Expr::Value(MalVal::Symbol(s))
    }

    pub fn string(s: String) -> Self {
        Expr::Value(MalVal::String(s))
    }

    pub fn keyword(s: String) -> Self {
        Expr::Value(MalVal::Keyword(s))
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
pub enum MalError<I> {
    Nom(nom::error::Error<I>),
    UnbalancedParens,
    Other(I),
}

impl<I: std::fmt::Debug, E> FromExternalError<I, E> for MalError<I> {
    fn from_external_error(input: I, _kind: nom::error::ErrorKind, _e: E) -> Self {
        MalError::Other(input)
    }
}

impl<I> ParseError<I> for MalError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        MalError::Nom(nom::error::Error::new(input, kind))
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I: fmt::Display> fmt::Display for MalError<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalError::Nom(e) => write!(f, "Parse error at: {}", e.input),
            MalError::UnbalancedParens => write!(f, "Unbalanced parentheses"),
            MalError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl<I: fmt::Debug + fmt::Display> std::error::Error for MalError<I> {}
