// Protorun言語の文パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    sequence::{preceded, terminated},
};

use crate::protorun::ast::{Stmt, Span, Parameter, Decl};
use super::common::{ParseResult, ws_comments, identifier_string, with_context, calculate_span};
use super::types::parse_type;
use super::expressions::expression;

/// 文をパース
pub fn statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Stmt> {
    alt((
        |i| let_statement(i, original_input),
        |i| var_statement(i, original_input),
        |i| return_statement(i, original_input),
        map(
            |i| expression(i, original_input),
            move |expr| {
                let span = calculate_span(original_input, input);
                Stmt::Expr { expr, span }
            }
        )
    ))(input)
}

/// let文をパース
pub fn let_statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Stmt> {
    let (input, _) = ws_comments(tag("let"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    
    // 式をパース
    let (input, value) = with_context(
        "式の解析中にエラーが発生しました",
        cut(|i| expression(i, original_input))
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Stmt::Let {
        name,
        type_annotation,
        value,
        span,
    }))
}

/// var文をパース
pub fn var_statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Stmt> {
    let (input, _) = ws_comments(tag("var"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    
    // 式をパース
    let (input, value) = with_context(
        "式の解析中にエラーが発生しました",
        cut(|i| expression(i, original_input))
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Stmt::Var {
        name,
        type_annotation,
        value,
        span,
    }))
}

/// return文をパース
pub fn return_statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Stmt> {
    let (input, _) = ws_comments(tag("return"))(input)?;
    
    // 戻り値の式（オプション）
    let (input, value) = opt(|i| expression(i, original_input))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Stmt::Return {
        value,
        span,
    }))
}

/// 関数宣言をパース
pub fn function_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    let (input, _) = ws_comments(tag("fn"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // パラメータをパース
    let (input, parameters) = super::common::delimited_list(
        '(',
        |i| parameter(i, original_input),
        ',',
        ')'
    )(input)?;
    
    // 戻り値の型（オプション）
    let (input, return_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    
    // 関数本体
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = cut(|i| expression(i, original_input))(input)?;
    let (input, _) = opt(ws_comments(char(';')))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Decl::Function {
        name,
        parameters,
        return_type,
        body,
        span,
    }))
}

/// パラメータをパース
pub fn parameter<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Parameter> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Parameter {
        name,
        type_annotation,
        span,
    }))
}

/// プログラム全体をパース
pub fn program<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, crate::protorun::ast::Program> {
    use nom::character::complete::multispace0;
    use nom::multi::many0;
    use nom::sequence::terminated;
    
    let (input, _) = multispace0(input)?;
    
    // モジュール宣言をパース
    let (input, modules) = many0(terminated(
        |i| {
            let (i, _) = multispace0(i)?;
            super::modules::parse_module(i, original_input)
        },
        multispace0
    ))(input)?;
    
    // 関数宣言をパース
    let (input, declarations) = many0(terminated(
        |i| {
            let (i, _) = multispace0(i)?;
            function_declaration(i, original_input)
        },
        multispace0
    ))(input)?;
    
    // 型宣言をパース
    let (input, type_declarations) = many0(terminated(
        |i| {
            let (i, _) = multispace0(i)?;
            super::declarations::parse_type_declaration(i, original_input)
        },
        multispace0
    ))(input)?;
    
    // トレイト宣言をパース
    let (input, trait_declarations) = many0(terminated(
        |i| {
            let (i, _) = multispace0(i)?;
            super::declarations::parse_trait_declaration(i, original_input)
        },
        multispace0
    ))(input)?;
    
    // 実装宣言をパース
    let (input, impl_declarations) = many0(terminated(
        |i| {
            let (i, _) = multispace0(i)?;
            super::declarations::parse_impl_declaration(i, original_input)
        },
        multispace0
    ))(input)?;
    
    // 文をパース
    let (input, statements) = many0(
        terminated(
            |i| {
                let (i, _) = multispace0(i)?;
                statement(i, original_input)
            },
            ws_comments(char(';'))
        )
    )(input)?;
    
    let (input, _) = multispace0(input)?;
    
    Ok((input, crate::protorun::ast::Program {
        modules,
        declarations,
        type_declarations,
        trait_declarations,
        impl_declarations,
        statements,
    }))
}
