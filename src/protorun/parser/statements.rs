// Protorun言語の文パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    sequence::{preceded, terminated},
};

use crate::protorun::ast::{Stmt, Span, Parameter, Decl};
use super::common::{ParseResult, ParserContext, ws_comments, identifier_string, with_context};
use super::types::type_parser;
use super::expressions::expression;

/// パラメータをパース
pub fn parameter<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Parameter> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| type_parser(i, ctx)
        )
    )(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Parameter {
        name,
        type_annotation,
        span,
    }))
}

/// let文をパース
pub fn let_statement<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Stmt> {
    let (input, _) = ws_comments(tag("let"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| type_parser(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    
    // ここにコンテキストを追加
    let (input, value) = with_context(
        "式の解析中にエラーが発生しました",
        cut(|i| expression(i, ctx))
    )(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Stmt::Let {
        name,
        type_annotation,
        value,
        span,
    }))
}

/// 文をパース
pub fn statement<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Stmt> {
    alt((
        |i| let_statement(i, ctx),
        map(
            |i| expression(i, ctx),
            move |expr| {
                let span = ctx.calculate_span(input);
                Stmt::Expr { expr, span }
            }
        )
    ))(input)
}

/// 関数宣言をパース
pub fn function_declaration<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Decl> {
    let (input, _) = ws_comments(tag("fn"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, parameters) = super::common::delimited_list(
        '(',
        |i| parameter(i, ctx),
        ',',
        ')'
    )(input)?;
    let (input, return_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| type_parser(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = cut(|i| expression(i, ctx))(input)?;
    let (input, _) = opt(ws_comments(char(';')))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Decl::Function {
        name,
        parameters,
        return_type,
        body,
        span,
    }))
}

/// プログラム全体をパース
pub fn program<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, crate::protorun::ast::Program> {
    use nom::character::complete::multispace0;
    use nom::multi::many0;
    
    let (input, _) = multispace0(input)?;
    let (input, declarations) = many0(|i| function_declaration(i, ctx))(input)?;
    let (input, statements) = many0(
        terminated(
            |i| statement(i, ctx),
            ws_comments(char(';'))
        )
    )(input)?;
    let (input, _) = multispace0(input)?;
    
    Ok((input, crate::protorun::ast::Program {
        declarations,
        statements,
    }))
}
