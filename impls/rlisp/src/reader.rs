use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_till, take_while1},
    character::complete::{char, digit1, multispace0, multispace1, newline},
    combinator::{map, map_res, opt, value},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

use crate::types::{Expr, MalVal};
use std::collections::HashMap;

// honestly I feel like the parser combinators are way more hassle than they are
// worth. I think doing a pratt parser by scratch would be easier to write and
// understand than this.

/// Parse a Scheme expression.
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let the_parser = alt((
        parse_list,
        parse_vector,
        parse_hashmap,
        parse_symbol,
        parse_keyword,
        parse_string,
        parse_number,
        parse_quasi,
        parse_quote,
        parse_splice_unquote, // must be before unquote because ~ ambiguity
        parse_unquote,
    ));

    // preceded(ws_or_comment, the_parser).parse(input)
    preceded(ws_or_comma, the_parser).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(ws_or_comma, digit1), |s: &str| {
        s.parse::<i64>().map(Expr::number)
    })
    .parse(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Expr> {
    preceded(
        ws_or_comma,
        map(
            take_while1(|c: char| !c.is_whitespace() && !"[]:(){}~`'\",".contains(c)),
            |s: &str| Expr::symbol(s.to_string()),
        ),
    )
    .parse(input)
}

fn parse_keyword(input: &str) -> IResult<&str, Expr> {
    preceded(
        ws_or_comma,
        preceded(
            char(':'),
            map(
                take_while1(|c: char| !c.is_whitespace() && !"[]:(){}~`'\",".contains(c)),
                |s: &str| Expr::keyword(s.to_string()),
            ),
        ),
    )
    .parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tag("\""),
            opt(escaped_transform(
                is_not("\\\""),
                '\\',
                alt((
                    value("\\\\", tag("\\")),
                    value("\\\"", tag("\"")),
                    value("\n", tag("n")),
                    value("\t", tag("t")),
                    // add more escape sequences as needed
                )),
            )),
            tag("\""), // closing quote
        ),
        |opt_str| Expr::string(opt_str.unwrap_or_default()),
    )
    .parse(input)
    // .map_err(|_| nom::Err::Failure(::UnbalancedParens))
}

fn parse_quasi(input: &str) -> IResult<&str, Expr> {
    map(preceded(char('`'), parse_expr), Expr::quasi).parse(input)
}

fn parse_quote(input: &str) -> IResult<&str, Expr> {
    map(preceded(char('\''), parse_expr), Expr::quote).parse(input)
}

fn parse_splice_unquote(input: &str) -> IResult<&str, Expr> {
    map(preceded(tag("~@"), parse_expr), Expr::splice_unquote).parse(input)
}

fn parse_unquote(input: &str) -> IResult<&str, Expr> {
    map(preceded(char('~'), parse_expr), Expr::unquote).parse(input)
}

fn parse_list(input: &str) -> IResult<&str, Expr> {
    let inner = separated_list0(ws_or_comma, parse_expr);
    map(
        delimited(
            preceded(ws_or_comma, char('(')),
            inner,
            preceded(ws_or_comma, char(')')),
        ),
        Expr::list,
    )
    .parse(input)
}

fn parse_vector(input: &str) -> IResult<&str, Expr> {
    let inner = separated_list0(ws_or_comma, parse_expr);
    map(
        delimited(
            preceded(ws_or_comma, char('[')),
            inner,
            preceded(ws_or_comma, char(']')),
        ),
        Expr::vector,
    )
    .parse(input)
}

fn parse_hashmap(input: &str) -> IResult<&str, Expr> {
    let p_val = alt((parse_number, parse_symbol, parse_string, parse_keyword));
    let inner = preceded(
        ws_or_comma,
        separated_list0(
            ws_or_comma,
            delimited(
                ws_or_comma,
                separated_pair(p_val, ws_or_comma, parse_expr),
                ws_or_comma,
            ),
        ),
    );

    map(
        delimited(char('{'), inner, char('}')),
        |pairs: Vec<(Expr, Expr)>| {
            let mut map = HashMap::new();
            for (k, v) in pairs {
                let key: MalVal = match k {
                    Expr::Value(v) => v,
                    _ => {
                        panic!("impossible: parsed a malval, but it wasn't a val!");
                    }
                };
                map.insert(key, Box::new(v));
            }
            Expr::HashMap(map)
        },
    )
    .parse(input)
}

fn parse_comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        preceded(char(';'), terminated(take_till(|c| c == '\n'), newline)),
    )
    .parse(input)
}

// many1 is needed so that the first parser can fail, many0 always succeeds
fn ws_or_comma(input: &str) -> IResult<&str, ()> {
    let p_comments = preceded(multispace1, parse_comment);
    alt((
        value((), many1(char(','))),
        value((), multispace0),
        value((), p_comments),
    ))
    .parse(input)
}
