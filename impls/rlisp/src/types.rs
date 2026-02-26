use nom::lib::std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MalError {
    UnknownIdent(String),
    SyntaxError(String),
    NotAFun(String),
    TypeError(String),
    Other(String),
}

impl MalError {
    pub fn unknown_ident<T>(s: &str) -> MalResult<T> {
        Err(MalError::UnknownIdent(String::from(s)))
    }
    pub fn syntax_error<T>(s: &str) -> MalResult<T> {
        Err(MalError::SyntaxError(String::from(s)))
    }
    pub fn other_error<T>(s: &str) -> MalResult<T> {
        Err(MalError::Other(String::from(s)))
    }
    pub fn not_afun_error<T>(s: &str) -> MalResult<T> {
        Err(MalError::NotAFun(String::from(s)))
    }
    pub fn type_error<T>(s: &str) -> MalResult<T> {
        Err(MalError::TypeError(String::from(s)))
    }
}

// type synonym over result
pub type MalResult<T> = Result<T, MalError>;

// From automatically converts from &MalError to MalError via clone
impl From<&MalError> for MalError {
    fn from(e: &MalError) -> Self {
        e.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MalVal {
    Number(i64),
    BSymbol(BuiltIn), // built in symbols
    Symbol(String),
    String(String),
    Keyword(String),
}

#[derive(Clone)]
pub enum Builtin {
    Unary(fn(Expr) -> MalResult<Expr>),
    Binary(fn(Expr, Expr) -> MalResult<Expr>),
    Nary(fn(&[Expr]) -> MalResult<Expr>),
}

impl MalVal {
    pub fn number(n: i64) -> Self {
        MalVal::Number(n)
    }

    pub fn symbol(s: String) -> Self {
        MalVal::Symbol(s)
    }

    pub fn builtin_symbol(s: BuiltIn) -> Self {
        MalVal::BSymbol(s)
    }

    pub fn string(s: String) -> Self {
        MalVal::String(s)
    }

    pub fn keyword(s: String) -> Self {
        MalVal::Keyword(s)
    }

    pub fn as_i64(&self) -> MalResult<i64> {
        let die = |s: &str, v: &String| {
            MalError::other_error(format!("as_i64: panic: {} {}", s, v).as_ref())
        };
        match self {
            MalVal::Number(i) => Ok(*i),
            MalVal::Symbol(i) => die("Got Symbol", i),
            MalVal::BSymbol(i) => die("Got BSymbol", &format!("{:?}", i)),
            MalVal::String(i) => die("Got String", i),
            MalVal::Keyword(i) => die("Got Keyword", i),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            MalVal::Symbol(i) => i.clone(),
            MalVal::BSymbol(i) => format!("{:?}", i),
            MalVal::String(i) => i.clone(),
            MalVal::Keyword(i) => i.clone(),
            MalVal::Number(i) => i.to_string(),
        }
    }
}

impl fmt::Display for MalVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MalVal::Number(n) => write!(f, "{}", n),
            MalVal::Symbol(s) => write!(f, "{}", s),
            MalVal::BSymbol(s) => write!(f, "{}", format!("{:?}", s)),
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
pub enum MalContainer {
    List(Vec<Box<Expr>>),
    Vector(Vec<Box<Expr>>),
    HashMap(HashMap<MalVal, Box<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(MalVal),
    Container(MalContainer),
    Quoted(QuoteKind, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Value(v) => write!(f, "{}", v),
            Expr::Container(MalContainer::List(e)) => {
                let ls = e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "({})", ls)
            }
            Expr::Container(MalContainer::Vector(e)) => {
                let ls = e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "[{}]", ls)
            }
            Expr::Container(MalContainer::HashMap(hmap)) => {
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

    pub fn builtin_symbol(s: BuiltIn) -> Self {
        Expr::Value(MalVal::builtin_symbol(s))
    }

    pub fn string(s: String) -> Self {
        Expr::Value(MalVal::string(s))
    }

    pub fn keyword(s: String) -> Self {
        Expr::Value(MalVal::keyword(s))
    }

    pub fn list(es: Vec<Expr>) -> Self {
        Expr::Container(MalContainer::List(
            es.into_iter().map(|e| Box::new(e)).collect(),
        ))
    }

    pub fn vector(es: Vec<Expr>) -> Self {
        Expr::Container(MalContainer::Vector(es.into_iter().map(Box::new).collect()))
    }

    pub fn hash_map(es: HashMap<MalVal, Box<Expr>>) -> Self {
        Expr::Container(MalContainer::HashMap(es))
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

    pub fn as_i64(&self) -> MalResult<i64> {
        match self {
            Expr::Value(v) => v.as_i64(),
            _ => MalError::type_error(format!("Wanted: i64, Got: {}", self).as_ref()),
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// This order is important and determines the lookup in lib::OP_TABLE
// this must correspond one-for-one with that table
pub enum BuiltIn {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltIn::Add => write!(f, "+"),
            BuiltIn::Sub => write!(f, "-"),
            BuiltIn::Div => write!(f, "/"),
            BuiltIn::Mul => write!(f, "*"),
        }
    }
}
