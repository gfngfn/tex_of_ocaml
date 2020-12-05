use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    IResult,
};
use std::char;

use crate::syntax::{Expr, Ident};

type Input = str;

pub type Error<'a> = nom::Err<nom::error::Error<&'a Input>>;

pub fn parse<'a>(s: &'a str) -> Result<Expr, Error<'a>> {
    match parse_main(s) {
        Ok((_, e)) => Ok(e), /* TODO: check emptiness */
        Err(err) => Err(err),
    }
}

fn skip_space(s: &Input) -> IResult<&Input, ()> {
    let (s, _) = take_while(char::is_whitespace)(s)?;
    Ok((s, ()))
}

fn parse_main(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_abstraction, parse_application))(s)
}

fn parse_abstraction(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("fun")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, x) = parse_ident(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("->")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e) = parse_main(s)?;
    Ok((s, Expr::Lambda(x, Box::new(e))))
}

fn parse_application(s: &Input) -> IResult<&Input, Expr> {
    (parse_single)(s) /* TODO */
}

fn parse_single(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_variable, parse_paren))(s)
}

fn parse_variable(s: &Input) -> IResult<&Input, Expr> {
    let (s, x) = parse_ident(s)?;
    Ok((s, Expr::Var(x)))
}

fn parse_ident(s: &Input) -> IResult<&Input, Ident> {
    let (s, alphas) = take_while1(char::is_alphabetic)(s)?;
    Ok((s, Ident::of_string(alphas.to_string())))
}

fn parse_paren(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e) = parse_main(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, e))
}
