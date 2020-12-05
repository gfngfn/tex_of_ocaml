use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    IResult,
};
use std::char;

use crate::syntax::{Expr, Ident};

type Input = str;

pub type Error<'a> = nom::Err<&'a Input>;

pub fn parse(_s: &str) -> Result<Expr, Error<'static>> {
    Ok(Expr::Var(Ident::of_string("x".to_string())))
}

fn parse_main(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_abstraction, parse_application))(s)
}

fn parse_abstraction(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("fun")(s)?;
    let (s, x) = parse_ident(s)?;
    let (s, _) = tag("->")(s)?;
    let (s, e) = parse_main(s)?;
    Ok((s, Expr::Lambda(x, Box::new(e))))
}

fn parse_application(s: &Input) -> IResult<&Input, Expr> {
    (parse_single)(s)
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
    let (s, e) = parse_main(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, e))
}
