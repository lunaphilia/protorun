// Protorun言語のリテラル値パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, none_of},
    combinator::{map, map_res, opt, value},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::protorun::ast::{Expr, Span, LiteralValue};
use super::common::{ParseResult, ParserContext, ws_comments};

/// 整数リテラルをパース
pub fn int_literal(input: &str) -> ParseResult<i64> {
    map_res(
        recognize_int_literal,
        |s: &str| s.parse::<i64>()
    )(input)
}

/// 整数リテラルを認識
pub fn recognize_int_literal(input: &str) -> ParseResult<&str> {
    nom::combinator::recognize(
        pair(
            opt(char('-')),
            digit1
        )
    )(input)
}

/// 浮動小数点リテラルをパース
pub fn float_literal(input: &str) -> ParseResult<f64> {
    map_res(
        nom::combinator::recognize(
            tuple((
                opt(char('-')),
                digit1,
                char('.'),
                digit1
            ))
        ),
        |s: &str| s.parse::<f64>()
    )(input)
}

/// 文字列リテラルをパース
pub fn string_literal(input: &str) -> ParseResult<String> {
    delimited(
        char('"'),
        map(
            many0(
                alt((
                    map(tag("\\n"), |_| '\n'),
                    map(tag("\\r"), |_| '\r'),
                    map(tag("\\t"), |_| '\t'),
                    map(tag("\\\\"), |_| '\\'),
                    map(tag("\\\""), |_| '"'),
                    none_of("\"\\")
                ))
            ),
            |chars| chars.into_iter().collect()
        ),
        char('"')
    )(input)
}

/// 真偽値リテラルをパース
pub fn bool_literal(input: &str) -> ParseResult<bool> {
    alt((
        value(true, tag("true")),
        value(false, tag("false"))
    ))(input)
}

/// 整数リテラル式をパース
pub fn int_literal_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (remaining, value) = int_literal(input)?;
    let span = ctx.calculate_span(remaining);
    
    Ok((remaining, Expr::IntLiteral(value, span)))
}

/// 浮動小数点リテラル式をパース
pub fn float_literal_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (remaining, value) = float_literal(input)?;
    let span = ctx.calculate_span(remaining);
    
    Ok((remaining, Expr::FloatLiteral(value, span)))
}

/// 文字列リテラル式をパース
pub fn string_literal_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (remaining, value) = string_literal(input)?;
    let span = ctx.calculate_span(remaining);
    
    Ok((remaining, Expr::StringLiteral(value, span)))
}

/// 真偽値リテラル式をパース
pub fn bool_literal_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (remaining, value) = bool_literal(input)?;
    let span = ctx.calculate_span(remaining);
    
    Ok((remaining, Expr::BoolLiteral(value, span)))
}

/// ユニットリテラル式をパース
pub fn unit_literal_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (remaining, _) = ws_comments(tag("()"))(input)?;
    let span = ctx.calculate_span(remaining);
    
    Ok((remaining, Expr::UnitLiteral(span)))
}

/// リテラルパターン値をパース
pub fn literal_pattern_value<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, LiteralValue> {
    alt((
        map(int_literal, LiteralValue::Int),
        map(float_literal, LiteralValue::Float),
        map(string_literal, LiteralValue::String),
        map(bool_literal, LiteralValue::Bool),
        map(ws_comments(tag("()")), |_| LiteralValue::Unit)
    ))(input)
}

use nom::sequence::tuple;
