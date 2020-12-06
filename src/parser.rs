use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::{map, opt},
    error::{make_error, ErrorKind},
    sequence::tuple,
    IResult,
};
use std::char;

use crate::list::List;
use crate::syntax::{Const, Expr, Ident, Primitive};

type Input = str;

pub type Error<'a> = nom::Err<nom::error::Error<&'a Input>>;

pub fn parse<'a>(s: &'a str) -> Result<Expr, Error<'a>> {
    match tuple((skip_space, parse_main, skip_space))(s) {
        Ok((s, ((), e, ()))) => {
            if s.is_empty() {
                Ok(e)
            } else {
                Err(error(s))
            }
        }
        Err(err) => Err(err),
    }
}

fn error<'a>(s: &'a Input) -> Error<'a> {
    nom::Err::Error(make_error(s, ErrorKind::Verify))
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
    let (s, res) = parse_ident(s)?;
    let x: Ident = res.map_err(|_| error(s))?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("->")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e) = parse_main(s)?;
    Ok((s, Expr::Lambda(x, Box::new(e))))
}

fn parse_application(s: &Input) -> IResult<&Input, Expr> {
    let (s, (e, eargs)) = parse_single_list(s)?;
    let eret: Expr = eargs.foldl(|eapp, earg| Expr::Apply(Box::new(eapp), Box::new(earg)), e);
    Ok((s, eret))
}

fn parse_single_list<'a>(s: &'a Input) -> IResult<&'a Input, (Expr, List<Expr>)> {
    map(
        tuple((parse_single, opt(tuple((skip_space, parse_single_list))))),
        |(e, tail_opt)| match tail_opt {
            Some(((), (e2, eargs))) => (e, List::Cons(e2, Box::new(eargs))),
            None => (e, List::Nil),
        },
    )(s)
}

fn parse_single(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_variable, parse_constant, parse_paren))(s)
}

fn parse_variable(s: &Input) -> IResult<&Input, Expr> {
    let (s, res) = parse_ident(s)?;
    match res {
        Ok(x) => Ok((s, Expr::Var(x))),
        Err(prim) => Ok((s, Expr::Primitive(prim))),
    }
}

fn parse_ident(s: &Input) -> IResult<&Input, Result<Ident, Primitive>> {
    let (s, opt) = map(take_while1(char::is_alphabetic), |alphas| match alphas {
        "fun" => None,
        "add" => Some(Err(Primitive::Add)),
        "sub" => Some(Err(Primitive::Sub)),
        "mult" => Some(Err(Primitive::Mult)),
        "append" => Some(Err(Primitive::Append)),
        alphas => Some(Ok(Ident::of_string(alphas.to_string()))),
    })(s)?;
    match opt {
        None => Err(error(s)),
        Some(either) => Ok((s, either)),
    }
}

fn parse_paren(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("(")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e) = parse_main(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, e))
}

fn parse_constant(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_integer_literal, parse_string_literal))(s)
}

fn parse_integer_literal(s: &Input) -> IResult<&Input, Expr> {
    let (s, nums) = take_while1(char::is_numeric)(s)?;
    let n: i32 = nums.parse().unwrap();
    Ok((s, Expr::Const(Const::Int(n))))
}

fn parse_string_literal(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("\"")(s)?;
    let (s, contents) = take_while(|ch| ch != '"')(s)?;
    let (s, _) = tag("\"")(s)?;
    let contents = contents.to_string();
    Ok((s, Expr::Const(Const::String(contents))))
}
