use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_till, take_while1},
    character::complete::{char, digit1, multispace0, newline},
    combinator::{map, map_res, opt, value},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

use crate::types::Expr;

// honestly I feel like the parser combinators are way more hassle than they are
// worth. I think doing a pratt parser by scratch would be easier to write and
// understand than this.

/// Parse a Scheme expression.
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let the_parser = alt((
        parse_list,
        parse_symbol,
        parse_string,
        parse_number,
        parse_quasi,
        parse_quote,
        parse_splice_unquote, // must be before unquote because ~ ambiguity
        parse_unquote,
        // parse_comment,
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
            take_while1(|c: char| !c.is_whitespace() && !"()~`'\",".contains(c)),
            |s: &str| Expr::symbol(s.to_string()),
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
        Expr::List,
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
    alt((value((), many1(char(','))), value((), multispace0))).parse(input)
}
