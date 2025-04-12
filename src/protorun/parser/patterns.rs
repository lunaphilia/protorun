// Protorun言語のパターンマッチングパーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, cut, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded},
};

use crate::protorun::ast::{Pattern, LiteralValue, Expr}; // Expr をインポート
use super::common::{ParseResult, ws_comments, identifier_string, calculate_span, consume_ws_comments}; // consume_ws_comments をインポート
use super::literals::{literal_pattern_value};
use super::expressions::parse_guard_expression; // parse_guard_expression をインポート

/// パターンをパース
pub fn pattern<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Pattern> {
    ws_comments(
        alt((
            // リテラルパターン
            map(
                |i| literal_pattern_value(i, original_input),
                move |value| {
                    let span = calculate_span(original_input, input);
                    Pattern::Literal(value, span)
                }
            ),
            // ワイルドカードパターン
            map(
                ws_comments(tag("_")),
                move |_| {
                    let span = calculate_span(original_input, input);
                    Pattern::Wildcard(span)
                }
            ),
            // タプルパターン
            map(
                |i| parse_tuple_pattern(i, original_input),
                move |(patterns, remaining)| {
                    let span = calculate_span(original_input, remaining);
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
                    |i| parse_constructor_args(i, original_input)
                ),
                move |(name, (args, remaining))| {
                    let span = calculate_span(original_input, remaining);
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
                    let span = calculate_span(original_input, input);
                    Pattern::Identifier(name, span)
                }
            )
        ))
    )(input)
}

/// タプルパターンをパース
fn parse_tuple_pattern<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, (Vec<Pattern>, &'a str)> {
    let (remaining, patterns) = delimited(
        ws_comments(char('(')),
        separated_list0(
            ws_comments(char(',')),
            |i| pattern(i, original_input)
        ),
        cut(ws_comments(char(')')))
    )(input)?;
    
    Ok((remaining, (patterns, remaining)))
}

/// コンストラクタ引数をパース
fn parse_constructor_args<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, (Vec<Pattern>, &'a str)> {
    let (remaining, patterns) = delimited(
        ws_comments(char('(')),
        separated_list0(
            ws_comments(char(',')),
            |i| pattern(i, original_input)
        ),
        cut(ws_comments(char(')')))
    )(input)?;
    
    Ok((remaining, (patterns, remaining)))
}

/// match式のケースをパース（式パーサーを引数として受け取る）
pub fn match_case<'a, F>(
    input: &'a str, 
    original_input: &'a str,
    expression_parser: F
) -> ParseResult<'a, (Pattern, Option<crate::protorun::ast::Expr>, crate::protorun::ast::Expr)> 
where
    F: Fn(&'a str, &'a str) -> ParseResult<'a, Expr> // crate::protorun::ast::Expr を Expr に
{
    let (input_after_pat, pat) = pattern(input, original_input)?;

    let (input_after_guard, guard) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| {
                // ガード節のパースには専用のパーサーを使用
                parse_guard_expression(i, original_input)
            }
        )
    )(input_after_pat)?; // Use input_after_pat

    let (input_after_arrow, _) = ws_comments(tag("=>"))(input_after_guard)?; // Use input_after_guard

    let (input_after_expr, expr) = expression_parser(input_after_arrow, original_input)?; // Use input_after_arrow

    // 式の後の空白/コメントを消費する
    let (input_after_ws, _) = consume_ws_comments(input_after_expr)?;

    Ok((input_after_ws, (pat, guard, expr))) // Return input_after_ws
}
