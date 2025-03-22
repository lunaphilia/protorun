// Protorun言語の文パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    sequence::{preceded, terminated},
};

use crate::protorun::ast::{Stmt, Span, Parameter, Decl};
use crate::protorun::symbol::{Symbol, SymbolKind, ScopeKind};
use super::common::{ParseResult, ParserContext, ws_comments, identifier_string, with_context};
use super::types::parse_type;
use super::expressions::expression;

/// パラメータをパース
pub fn parameter<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Parameter> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, ctx)
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
            |i| parse_type(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    
    // ここにコンテキストを追加
    let (input, value) = with_context(
        "式の解析中にエラーが発生しました",
        cut(|i| expression(i, ctx))
    )(input)?;
    
    let span = ctx.calculate_span(input);
    
    // シンボルテーブルに変数を登録
    let symbol = Symbol {
        name: name.clone(),
        kind: SymbolKind::Variable,
        type_annotation: type_annotation.clone(),
        declaration_span: span.clone(),
        is_mutable: false, // 将来的にはmut修飾子をサポート
        type_info: None,
        is_used: false,
    };
    
    // シンボル登録（エラーは無視して構文解析を続行）
    let _ = ctx.add_symbol(symbol);
    
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
pub fn function_declaration<'a>(input: &'a str, ctx: &mut ParserContext<'a>) -> ParseResult<'a, Decl> {
    let (input, _) = ws_comments(tag("fn"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // 関数名をシンボルテーブルに登録
    let func_span = ctx.calculate_span(input);
    let func_symbol = Symbol {
        name: name.clone(),
        kind: SymbolKind::Function,
        type_annotation: None, // 関数の型は後で構築
        declaration_span: func_span.clone(),
        is_mutable: false,
        type_info: None,
        is_used: false,
    };
    let _ = ctx.add_symbol(func_symbol);
    
    // 関数スコープを開始
    ctx.enter_scope(ScopeKind::Function);
    
    let (input, parameters) = super::common::delimited_list(
        '(',
        |i| parameter(i, ctx),
        ',',
        ')'
    )(input)?;
    
    // パラメータをシンボルテーブルに登録
    for param in &parameters {
        let param_symbol = Symbol {
            name: param.name.clone(),
            kind: SymbolKind::Parameter,
            type_annotation: param.type_annotation.clone(),
            declaration_span: param.span.clone(),
            is_mutable: false,
            type_info: None,
            is_used: false,
        };
        let _ = ctx.add_symbol(param_symbol);
    }
    
    let (input, return_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = cut(|i| expression(i, ctx))(input)?;
    let (input, _) = opt(ws_comments(char(';')))(input)?;
    
    let span = ctx.calculate_span(input);
    
    // 関数スコープを終了
    ctx.exit_scope();
    
    Ok((input, Decl::Function {
        name,
        parameters,
        return_type,
        body,
        span,
    }))
}

/// プログラム全体をパース
pub fn program<'a>(input: &'a str, ctx: &mut ParserContext<'a>) -> ParseResult<'a, crate::protorun::ast::Program> {
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
