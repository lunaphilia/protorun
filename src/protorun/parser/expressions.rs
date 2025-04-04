// Protorun言語の式パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{cut, map, opt, value},
    error::VerboseError,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated}, // terminated を追加
};

// BlockItem をインポート, HandlerSpec を削除
use crate::protorun::ast::{Expr, BinaryOperator, UnaryOperator, ComprehensionKind, BlockItem};
use super::common::{ParseResult, ws_comments, identifier_string, delimited_list, calculate_span};
use super::literals::{int_literal_expr, float_literal_expr, string_literal_expr, bool_literal_expr, unit_literal_expr};
use super::patterns::pattern;
use super::types::parse_type;
// statement と parse_declaration をインポート
use super::statements::statement;
use super::declarations::parse_declaration;

/// 括弧式をパース
pub fn paren_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // ラムダ式のパターンに一致する場合のチェックは不要
    // lambda_exprがparen_exprよりも先に試されるため
    
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    
    // 空の括弧 -> ユニットリテラル
    if let Ok((_, _)) = char::<&str, VerboseError<&str>>(')')(&input) {
        let (input, _) = char(')')(input)?;
        let span = calculate_span(original_input, input);
        return Ok((input, Expr::UnitLiteral(span)));
    }
    
    // 括弧内の式をパース
    let (input, expr) = ws_comments(|i| expression(i, original_input))(input)?;
    let (input, _) = cut(ws_comments(char(')')))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::ParenExpr(Box::new(expr), span)))
}

/// 基本式をパース
pub fn primary<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let result = ws_comments(
        alt((
            // 括弧式
            |i| paren_expr(i, original_input),
            // 整数リテラル
            |i| int_literal_expr(i, original_input),
            // 浮動小数点リテラル
            |i| float_literal_expr(i, original_input),
            // 文字列リテラル
            |i| string_literal_expr(i, original_input),
            // 真偽値リテラル
            |i| bool_literal_expr(i, original_input),
            // ユニットリテラル
            |i| unit_literal_expr(i, original_input),
            // 識別子
            map(
                identifier_string,
                move |name| {
                    let span = calculate_span(original_input, input);
                    Expr::Identifier(name, span)
                }
            ),
            // ブロック式
            |i| block_expr(i, original_input)
        ))
    )(input);
    
    result
}

/// ブロック式をパース: { (Declaration | Statement | Expression)* }
/// ブロックの値は、意味解析/実行時に最後の要素が式ならその値、そうでなければ Unit となる。
pub fn block_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize; // Span計算用

    let (input, _) = ws_comments(char('{'))(input)?;

    // ブロック内の要素（宣言、文、または式）をパース
    let (input, items) = many0(
        // 各要素の後には空白が続くことを想定 (改行含む)
        terminated(
            alt((
                map(|i| parse_declaration(i, original_input), BlockItem::Declaration),
                map(|i| statement(i, original_input), BlockItem::Statement), // statement は Return のみ
                map(|i| expression(i, original_input), BlockItem::Expression), // 副作用のための式もパース
            )),
            multispace0 // 要素間の空白を消費
        )
    )(input)?;

    // 閉じ括弧 '}' をパース
    let (input, _) = cut(ws_comments(char('}')))(input)?;

    let span = calculate_span(original_input, input); // TODO: Span 計算の正確性を確認・修正する。

    // 空のブロック {} は UnitLiteral として扱う (意味的には BlockExpr{ items: [] } でも良いが、簡潔化のため)
    if items.is_empty() {
        return Ok((input, Expr::UnitLiteral(span)));
    }

    Ok((input, Expr::BlockExpr {
        items, // final_expr は削除
        span,
    }))
}

/// 後置式（関数呼び出しとメンバーアクセス）をパース
pub fn postfix<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (mut current_input, mut expr) = primary(input, original_input)?;
    
    // 関数呼び出しとメンバーアクセスを繰り返しパース
    loop {
        // 関数呼び出し
        if let Ok((new_input, args)) = delimited(
            ws_comments(char('(')),
            separated_list0(
                ws_comments(char(',')),
                |i| expression(i, original_input)
            ),
            cut(ws_comments(char(')')))
        )(current_input) {
            let span = calculate_span(original_input, new_input);
            expr = Expr::FunctionCall {
                function: Box::new(expr),
                arguments: args,
                span,
            };
            current_input = new_input;
            continue;
        }
        
        // メンバーアクセス
        if let Ok((new_input, _)) = ws_comments(char('.'))(current_input) {
            if let Ok((new_input, member)) = ws_comments(identifier_string)(new_input) {
                let span = calculate_span(original_input, new_input);
                expr = Expr::MemberAccess {
                    object: Box::new(expr),
                    member,
                    span,
                };
                current_input = new_input;
                continue;
            }
        }
        
        // どちらもマッチしなければループを抜ける
        break;
    }
    
    Ok((current_input, expr))
}

/// 単項演算をパース
pub fn unary<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    alt((
        map(
            pair(
                ws_comments(char('-')),
                |i| unary(i, original_input)
            ),
            move |(_, expr)| {
                let span = calculate_span(original_input, input);
                Expr::UnaryOp {
                    operator: UnaryOperator::Neg,
                    expr: Box::new(expr),
                    span,
                }
            }
        ),
        map(
            pair(
                ws_comments(char('!')),
                |i| unary(i, original_input)
            ),
            move |(_, expr)| {
                let span = calculate_span(original_input, input);
                Expr::UnaryOp {
                    operator: UnaryOperator::Not,
                    expr: Box::new(expr),
                    span,
                }
            }
        ),
        |i| postfix(i, original_input)
    ))(input)
}

/// 因子をパース（乗除算）
pub fn factor<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = unary(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Mul, char('*')),
                value(BinaryOperator::Div, char('/')),
                value(BinaryOperator::Mod, char('%'))
            ))),
            |i| unary(i, original_input)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (op, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    });
    
    Ok((input, result))
}

/// 項をパース（加減算）
pub fn term<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = factor(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Add, char('+')),
                value(BinaryOperator::Sub, char('-'))
            ))),
            |i| factor(i, original_input)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (op, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    });
    
    Ok((input, result))
}

/// 比較演算をパース
pub fn comparison<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = term(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                // 2文字演算子を先に試す
                value(BinaryOperator::Lte, tag("<=")),
                value(BinaryOperator::Gte, tag(">=")),
                // 1文字演算子は後
                value(BinaryOperator::Lt, tag("<")),
                value(BinaryOperator::Gt, tag(">"))
            ))),
            |i| term(i, original_input)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

/// 等価演算をパース
pub fn equality<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = comparison(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Eq, tag("==")),
                value(BinaryOperator::Neq, tag("!="))
            ))),
            |i| comparison(i, original_input)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

/// 論理AND演算をパース
pub fn logical_and<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = equality(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("&&")),
            |i| equality(i, original_input)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (_, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: BinaryOperator::And,
            right: Box::new(right),
            span,
        }
    })))
}

/// 論理OR演算をパース
pub fn logical_or<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, first) = logical_and(input, original_input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("||")),
            |i| logical_and(i, original_input)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (_, right)| {
        let span = calculate_span(original_input, input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: BinaryOperator::Or,
            right: Box::new(right),
            span,
        }
    });
    
    Ok((input, result))
}

/// if式をパース
pub fn if_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("if"))(input)?;
    let (input, condition) = expression(input, original_input)?;
    let (input, then_branch) = block_expr(input, original_input)?;
    let (input, else_branch) = opt(
        preceded(
            ws_comments(tag("else")),
            alt((
                |i| if_expr(i, original_input),
                |i| block_expr(i, original_input)
            ))
        )
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::IfExpr {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: else_branch.map(Box::new),
        span,
    }))
}

/// match式をパース
pub fn match_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("match"))(input)?;
    let (input, scrutinee) = expression(input, original_input)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, cases) = separated_list0(
        ws_comments(char(',')),
        |i| super::patterns::match_case(i, original_input, expression)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = cut(ws_comments(char('}')))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::MatchExpr {
        scrutinee: Box::new(scrutinee),
        cases,
        span,
    }))
}

/// リスト内包表記をパース
pub fn list_comprehension<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, output_expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, original_input)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, original_input)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, original_input)
        )
    )(input)?;
    let (input, _) = ws_comments(char(']'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::CollectionComprehension {
        kind: ComprehensionKind::List,
        output_expr: Box::new(output_expr),
        input_expr: Box::new(input_expr),
        pattern: pat,
        condition: condition.map(Box::new),
        span,
    }))
}

/// マップ内包表記をパース
pub fn map_comprehension<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, key_expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(tag("->"))(input)?;
    let (input, value_expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, original_input)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, original_input)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, original_input)
        )
    )(input)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    // マップ内包表記は、キーと値のペアを出力する特殊なケース
    // 内部的には、タプル式を出力するリスト内包表記として扱う
    let span = calculate_span(original_input, input);
    
    // キーと値のペアを表すタプル式を作成
    let output_expr = Expr::ParenExpr(
        Box::new(Expr::BinaryOp {
            left: Box::new(key_expr),
            operator: BinaryOperator::Add, // 実際にはタプルを表すための仮のオペレータ
            right: Box::new(value_expr),
            span: span.clone(),
        }),
        span.clone(),
    );
    
    Ok((input, Expr::CollectionComprehension {
        kind: ComprehensionKind::Map,
        output_expr: Box::new(output_expr),
        input_expr: Box::new(input_expr),
        pattern: pat,
        condition: condition.map(Box::new),
        span,
    }))
}

/// セット内包表記をパース
pub fn set_comprehension<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("#{"))(input)?;
    let (input, output_expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, original_input)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, original_input)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, original_input)
        )
    )(input)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::CollectionComprehension {
        kind: ComprehensionKind::Set,
        output_expr: Box::new(output_expr),
        input_expr: Box::new(input_expr),
        pattern: pat,
        condition: condition.map(Box::new),
        span,
    }))
}

/// コレクション内包表記をパース（統合版）
pub fn collection_comprehension<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    alt((
        |i| list_comprehension(i, original_input),
        |i| map_comprehension(i, original_input),
        |i| set_comprehension(i, original_input)
    ))(input)
}

/// bind式のバインド文をパース
pub fn bind_statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, (crate::protorun::ast::Pattern, Expr)> {
    let (input, pat) = pattern(input, original_input)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(char(';'))(input)?;
    
    Ok((input, (pat, expr)))
}

/// bind式をパース
pub fn bind_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("bind"))(input)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    
    let (input, bindings) = many0(|i| bind_statement(i, original_input))(input)?;
    
    let (input, final_expr) = expression(input, original_input)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::BindExpr {
        bindings,
        final_expr: Box::new(final_expr),
        span,
    }))
}

/// with式をパース
pub fn with_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("with"))(input)?;

    // ハンドラ式をパース (alt と HandlerSpec を削除)
    let (input, handler_expr) = logical_or(input, original_input)?;
    let handler = Box::new(handler_expr);

    // オプションの効果型
    let (input, effect_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    
    // 本体（ブロック式）
    let (input, body) = block_expr(input, original_input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::WithExpr {
        handler,
        effect_type,
        body: Box::new(body),
        span,
    }))
}

/// リストリテラルをパース
pub fn list_literal<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, elements) = separated_list0(
        ws_comments(char(',')),
        |i| expression(i, original_input)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char(']'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::ListLiteral {
        elements,
        span,
    }))
}

/// マップリテラルをパース
pub fn map_literal<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, entries) = separated_list0(
        ws_comments(char(',')),
        |i| {
            let (i, key) = expression(i, original_input)?;
            let (i, _) = ws_comments(tag("->"))(i)?;
            let (i, value) = expression(i, original_input)?;
            Ok((i, (key, value)))
        }
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::MapLiteral {
        entries,
        span,
    }))
}

/// セットリテラルをパース
pub fn set_literal<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("#{"))(input)?;
    let (input, elements) = separated_list0(
        ws_comments(char(',')),
        |i| expression(i, original_input)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Expr::SetLiteral {
        elements,
        span,
    }))
}

/// コレクションリテラルをパース（統合版）
pub fn collection_literal<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    alt((
        |i| list_literal(i, original_input),
        |i| map_literal(i, original_input),
        |i| set_literal(i, original_input)
    ))(input)
}

/// パラメータをパース
fn parameter<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, crate::protorun::ast::Parameter> {
    // 識別子をパース
    let (input, name) = identifier_string(input)?;
    
    // オプションの型注釈をパース
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| super::types::parse_type(i, original_input)
        )
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, crate::protorun::ast::Parameter {
        name,
        type_annotation,
        span,
    }))
}

/// ラムダ式をパース
pub fn lambda_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // 入力が既に'('で始まっているかどうかを確認
    if input.starts_with('(') {
        // 先読みでラムダ式かどうかを確認
        let (params_input, _) = char('(')(input)?;

        let params_result = separated_list0(
            ws_comments(char(',')),
            |i| parameter(i, original_input)
        )(params_input);

        if let Ok((params_rest, _params)) = params_result { // params -> _params
            let close_paren_result = ws_comments(char(')'))(params_rest);
            if let Ok((after_paren, _)) = close_paren_result {
                let arrow_result = ws_comments(tag("=>"))(after_paren);
                if let Ok((_, _)) = arrow_result {
                    // ラムダ式と確認できたので、パースを続行
                    
                    // 通常のパラメータリストのパース
                    let delimited_result = delimited_list(
                        '(',
                        |i| parameter(i, original_input),
                        ',',
                        ')'
                    )(input);
                    
                    match delimited_result {
                        Ok((input, parameters)) => {
                            // "=>"トークンをパース
                            match ws_comments(tag("=>"))(input) {
                                Ok((input, _)) => {
                                    // 本体の式をパース
                                    match expression(input, original_input) {
                                        Ok((input, body)) => {
                                            let span = calculate_span(original_input, input);
                                            
                                            return Ok((input, Expr::LambdaExpr {
                                                parameters,
                                                body: Box::new(body),
                                                span,
                                            }));
                                        },
                                        Err(e) => {
                                            return Err(e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }
        
        // ラムダ式ではないと判断
        return Err(nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Nom(nom::error::ErrorKind::Tag))] }));
    } else {
        // 既に'('が消費されている場合は、パラメータリストを直接パース
        
        // パラメータリストをパース（'('は既に消費されている）
        match separated_list0(
            ws_comments(char(',')),
            |i| parameter(i, original_input)
        )(input) {
            Ok((input, params)) => {
                match ws_comments(char(')'))(input) {
                    Ok((input, _)) => {
                        match ws_comments(tag("=>"))(input) {
                            Ok((input, _)) => {
                                match expression(input, original_input) {
                                    Ok((input, body)) => {
                                        let span = calculate_span(original_input, input);
                                        
                                        Ok((input, Expr::LambdaExpr {
                                            parameters: params,
                                            body: Box::new(body),
                                            span,
                                        }))
                                    },
                                    Err(e) => {
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                Err(e)
                            }
                        }
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

/// 式をパース
pub fn expression<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // 特殊な式と論理OR演算（最も優先度の低い演算子）をパース
    alt((
        |i| if_expr(i, original_input),
        |i| match_expr(i, original_input),
        |i| bind_expr(i, original_input),
        |i| with_expr(i, original_input),
        |i| lambda_expr(i, original_input),
        |i| collection_literal(i, original_input),
        |i| collection_comprehension(i, original_input),
        |i| logical_or(i, original_input)
    ))(input)
}
