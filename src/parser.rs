use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::{map, opt},
    error::{make_error, ErrorKind},
    sequence::tuple,
    IResult,
};
use std::char;

use crate::syntax::{Const, Expr, Ident, Primitive};

enum IdentResult {
    Primitive(Primitive),
    Const(Const),
    Ident(Ident),
}

type Input = str;

#[derive(Debug)]
pub enum Error {
    NotCompletelyConsumed(String),
    Other(String),
}

pub fn parse(input: &str) -> Result<Expr, Error> {
    match tuple((skip_space, parse_main, skip_space))(input) {
        Ok((s, ((), e, ()))) => {
            if s.is_empty() {
                Ok(e)
            } else {
                Err(Error::NotCompletelyConsumed(s.to_string()))
            }
        }
        Err(err) => {
            // Due to the lifetime limitation,
            // `err` cannot be directly returned here.
            Err(Error::Other(format!("{}", err)))
        }
    }
}

fn error(s: &str, kind: ErrorKind) -> nom::Err<nom::error::Error<&str>> {
    nom::Err::Error(make_error(s, kind))
}

fn skip_space(s: &Input) -> IResult<&Input, ()> {
    let (s, _) = take_while(char::is_whitespace)(s)?;
    Ok((s, ()))
}

fn parse_main(s: &Input) -> IResult<&Input, Expr> {
    alt((
        parse_abstraction,
        parse_conditional,
        parse_let,
        parse_application,
    ))(s)
}

fn parse_abstraction(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("fun")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, ires) = parse_ident(s)?;
    let x: Ident = {
        match ires {
            IdentResult::Ident(x) => Ok(x),
            _ => Err(error(s, ErrorKind::Verify)),
        }
    }?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("->")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e) = parse_main(s)?;
    Ok((s, Expr::Lambda(x, Box::new(e))))
}

fn parse_conditional(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("if")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e0) = parse_main(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("then")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e1) = parse_main(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("else")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e2) = parse_main(s)?;
    Ok((s, Expr::If(Box::new(e0), Box::new(e1), Box::new(e2))))
}

fn parse_let(s: &Input) -> IResult<&Input, Expr> {
    let (s, _) = tag("let")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, ires) = parse_ident(s)?;
    let x: Ident = {
        match ires {
            IdentResult::Ident(x) => Ok(x),
            _ => Err(error(s, ErrorKind::Verify)),
        }
    }?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e1) = parse_main(s)?;
    let (s, _) = skip_space(s)?;
    let (s, _) = tag("in")(s)?;
    let (s, _) = skip_space(s)?;
    let (s, e2) = parse_main(s)?;
    Ok((
        s,
        Expr::Apply(Box::new(Expr::Lambda(x, Box::new(e2))), Box::new(e1)),
    ))
}

fn parse_application(s: &Input) -> IResult<&Input, Expr> {
    let (s, (mut e, eargs)) = parse_single_list(s)?;
    for earg in eargs {
        e = Expr::Apply(Box::new(e), Box::new(earg));
    }
    Ok((s, e))
}

fn parse_single_list(s: &Input) -> IResult<&Input, (Expr, Vec<Expr>)> {
    map(
        tuple((parse_single, opt(tuple((skip_space, parse_single_list))))),
        |(e, tail_opt)| match tail_opt {
            Some(((), (e2, mut eargs0))) => {
                let mut eargs = vec![e2];
                eargs.append(&mut eargs0);
                (e, eargs)
            }
            None => (e, Vec::new()),
        },
    )(s)
}

fn parse_single(s: &Input) -> IResult<&Input, Expr> {
    alt((parse_variable, parse_constant, parse_paren))(s)
}

fn parse_variable(s: &Input) -> IResult<&Input, Expr> {
    let (s, res) = parse_ident(s)?;
    match res {
        IdentResult::Ident(x) => Ok((s, Expr::Var(x))),
        IdentResult::Const(c) => Ok((s, Expr::Const(c))),
        IdentResult::Primitive(prim) => Ok((s, Expr::Primitive(prim))),
    }
}

fn parse_ident(s: &Input) -> IResult<&Input, IdentResult> {
    let (s, opt) = map(take_while1(char::is_alphabetic), |alphas| match alphas {
        "fun" => None,
        "let" => None,
        "in" => None,
        "if" => None,
        "then" => None,
        "else" => None,
        "true" => Some(IdentResult::Const(Const::Bool(true))),
        "false" => Some(IdentResult::Const(Const::Bool(false))),
        "add" => Some(IdentResult::Primitive(Primitive::Add)),
        "sub" => Some(IdentResult::Primitive(Primitive::Sub)),
        "mult" => Some(IdentResult::Primitive(Primitive::Mult)),
        "append" => Some(IdentResult::Primitive(Primitive::Append)),
        "arabic" => Some(IdentResult::Primitive(Primitive::Arabic)),
        "iszero" => Some(IdentResult::Primitive(Primitive::IsZero)),
        alphas => Some(IdentResult::Ident(Ident::new(alphas))),
    })(s)?;
    match opt {
        None => Err(error(s, ErrorKind::Verify)),
        Some(ires) => Ok((s, ires)),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ident(x: &str) -> Expr {
        Expr::Var(Ident::new(x))
    }

    fn apply(e1: Expr, e2: Expr) -> Expr {
        Expr::Apply(Box::new(e1), Box::new(e2))
    }

    fn lambda(x: &str, e: Expr) -> Expr {
        Expr::Lambda(Ident::new(x), Box::new(e))
    }

    fn int_const(n: i32) -> Expr {
        Expr::Const(Const::Int(n))
    }

    fn string_const(s: &str) -> Expr {
        Expr::Const(Const::String(s.to_string()))
    }

    #[test]
    fn parse_tests() {
        assert_eq!(int_const(42), parse("42").unwrap());
        assert_eq!(string_const("foo"), parse("\"foo\"").unwrap());
        assert_eq!(ident("x"), parse("x").unwrap());
        assert_eq!(
            apply(apply(ident("x"), ident("y")), ident("z")),
            parse("x y z").unwrap()
        );
        assert_eq!(
            lambda("x", apply(ident("x"), int_const(1))),
            parse("fun x -> x 1").unwrap()
        );
        assert_eq!(
            apply(lambda("x", apply(ident("x"), ident("y"))), ident("z")),
            parse("(fun x -> x y) z").unwrap()
        );
        assert_eq!(
            apply(lambda("x", ident("x")), int_const(1)),
            parse("let x = 1 in x").unwrap()
        );
    }
}
