use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::{map, opt, verify},
    sequence::tuple,
    IResult,
};
use std::char;

use crate::syntax::{Expr, Ident};

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, ref tail) => 1 + tail.len(),
        }
    }

    fn foldl<A>(self, f: fn(A, T) -> A, acc: A) -> A {
        match self {
            List::Nil => acc,
            List::Cons(x, tail) => tail.foldl(f, f(acc, x)),
        }
    }
}

/*
impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            List::Nil => None,
            List::Cons(x, tail) => {
                *self = tail;
                Some(x)
            }
        }
    }
}
*/

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

fn is_not_reserved(word: &str) -> bool {
    match word {
        "fun" => false,
        _ => true,
    }
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
    alt((parse_variable, parse_paren))(s)
}

fn parse_variable(s: &Input) -> IResult<&Input, Expr> {
    let (s, x) = parse_ident(s)?;
    Ok((s, Expr::Var(x)))
}

fn parse_ident(s: &Input) -> IResult<&Input, Ident> {
    let (s, alphas) = verify(take_while1(char::is_alphabetic), is_not_reserved)(s)?;
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
