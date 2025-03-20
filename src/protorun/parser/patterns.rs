// Protorun言語のパターンマッチングパーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, cut},
    multi::separated_list0,
    sequence::{delimited, pair},
};

use crate::protorun::ast::{Pattern, Span, LiteralValue};
use super::common::{ParseResult, ParserContext, ws_comments, identifier_string, delimited_list};
use super::literals::{literal_pattern_value};

/// パターンをパース
pub fn pattern<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Pattern> {
    ws_comments(
        alt((
            // リテラルパターン
            map(
                |i| literal_pattern_value(i, ctx),
                move |value| {
                    let span = ctx.calculate_span(input);
                    Pattern::Literal(value, span)
                }
            ),
            // ワイルドカードパターン
            map(
                ws_comments(tag("_")),
                move |_| {
                    let span = ctx.calculate_span(input);
                    Pattern::Wildcard(span)
                }
            ),
            // タプルパターン
            map(
                |i| parse_tuple_pattern(i, ctx),
                move |(patterns, remaining)| {
                    let span = ctx.calculate_span(remaining);
                    if patterns.is_empty() {
                        // 空のタプルはユニットとして扱う
                        Pattern::Literal(LiteralValue::Unit, span)
                    } else {
                        Pattern::Tuple(patterns, span)
                    }
                }
            ),
            // コンストラクタパターン（引数がある場合のみ）
            map(
                pair(
                    identifier_string,
                    |i| parse_constructor_args(i, ctx)
                ),
                move |(name, (args, remaining))| {
                    let span = ctx.calculate_span(remaining);
                    Pattern::Constructor {
                        name,
                        arguments: args,
                        span,
                    }
                }
            ),
            // 識別子パターン（最後に配置して他のパターンが優先されるようにする）
            map(
                identifier_string,
                move |name| {
                    let span = ctx.calculate_span(input);
                    Pattern::Identifier(name, span)
                }
            )
        ))
    )(input)
}

/// タプルパターンをパース
fn parse_tuple_pattern<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, (Vec<Pattern>, &'a str)> {
    let (remaining, patterns) = delimited(
        ws_comments(char('(')),
        separated_list0(
            ws_comments(char(',')),
            |i| pattern(i, ctx)
        ),
        cut(ws_comments(char(')')))
    )(input)?;
    
    Ok((remaining, (patterns, remaining)))
}

/// コンストラクタ引数をパース
fn parse_constructor_args<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, (Vec<Pattern>, &'a str)> {
    let (remaining, patterns) = delimited(
        ws_comments(char('(')),
        separated_list0(
            ws_comments(char(',')),
            |i| pattern(i, ctx)
        ),
        cut(ws_comments(char(')')))
    )(input)?;
    
    Ok((remaining, (patterns, remaining)))
}

/// match式のケースをパース（式パーサーを引数として受け取る）
pub fn match_case<'a, F>(
    input: &'a str, 
    ctx: &ParserContext<'a>,
    expression_parser: F
) -> ParseResult<'a, (Pattern, Option<crate::protorun::ast::Expr>, crate::protorun::ast::Expr)> 
where
    F: Fn(&'a str, &ParserContext<'a>) -> ParseResult<'a, crate::protorun::ast::Expr>
{
    let (input, pat) = pattern(input, ctx)?;
    let (input, guard) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression_parser(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(tag("=>"))(input)?;
    let (input, expr) = expression_parser(input, ctx)?;
    
    Ok((input, (pat, guard, expr)))
}

use nom::combinator::opt;
use nom::sequence::preceded;
