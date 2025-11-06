use nom::error::{ErrorKind, FromExternalError, ParseError};
use nom::lib::std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MalVal {
    Number(i64),
    Symbol(String),
    String(String),
    Keyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteKind {
    Quote,
    Unquote,
    Quasi,
    SpliceUnquote,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(MalVal),
    List(Vec<Expr>),
    Quoted(QuoteKind, Box<Expr>),
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
        Expr::List(es)
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
