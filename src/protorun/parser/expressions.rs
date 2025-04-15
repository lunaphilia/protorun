// Protorun言語の式パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{cut, map, opt, value}, // map_res は不要になった
    // error::{VerboseError, ErrorKind}, // ErrorKind, VerboseError は不要になった
    multi::{many0, separated_list0}, // separated_list1 は不要になった
    // sequence から重複を削除
    sequence::{delimited, pair, preceded, terminated},
};

// EffectParameter, Parameter, WithBinding をインポート
use crate::protorun::ast::{Expr, BinaryOperator, UnaryOperator, ComprehensionKind, BlockItem, EffectParameter, Parameter, WithBinding};
// keyword, parameter をインポート
use super::common::{ParseResult, ws_comments, identifier_string, delimited_list, calculate_span, keyword, parameter};
use super::literals::{int_literal_expr, float_literal_expr, string_literal_expr, bool_literal_expr, unit_literal_expr};
use super::patterns::pattern;
use super::types::parse_type;
// statement と parse_declaration をインポート
use super::statements::statement;
use super::declarations::parse_declaration;


// --- 新しいパーサー関数 ---

/// タプルリテラル (要素数 >= 2) をパース: (expr, expr, ...)
fn tuple_literal<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {

    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, elements) = delimited(
        ws_comments(char('(')),
        |i| {

            // Parse the first element
            let (i, first) = expression(i, original_input)?;

            // Parse the mandatory comma and second element
            let (i, _) = ws_comments(char(','))(i)?;

            let (i, second) = expression(i, original_input)?;

            // Parse optional subsequent elements (preceded by comma)
            let (i, rest) = many0(preceded(ws_comments(char(',')), |i| expression(i, original_input)))(i)?;


            let mut elements = vec![first, second];
            elements.extend(rest);
            Ok((i, elements))
        },
        cut(ws_comments(char(')'))) // Ensure closing parenthesis
    )(input)?;


    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);
    Ok((input, Expr::TupleLiteral { elements, span }))
}

/// グループ化式 (要素数 1) をパース: (expr)
fn grouped_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, expr) = delimited(
        ws_comments(char('(')),
        |i| expression(i, original_input),
        cut(ws_comments(char(')')))
    )(input)?;
    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);
    Ok((input, Expr::ParenExpr(Box::new(expr), span)))
}

// --- primary 関数の修正 ---

/// 基本式をパース
pub fn primary<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let result = ws_comments(
        alt((
            // 優先順位: タプル -> グループ化 -> ユニット -> その他リテラル/識別子/ブロック
            // タプルリテラル (要素数 >= 2)
            |i| tuple_literal(i, original_input),
            // グループ化式 (要素数 1)
            |i| grouped_expr(i, original_input),
            // ユニットリテラル ()
            |i| unit_literal_expr(i, original_input), // unit_literal_expr は () をパースする
            // 浮動小数点リテラル (整数より先に試す)
            |i| float_literal_expr(i, original_input),
            // 整数リテラル
            |i| int_literal_expr(i, original_input),
            // 文字列リテラル
            |i| string_literal_expr(i, original_input),
            // 真偽値リテラル
            |i| bool_literal_expr(i, original_input),
            // 識別子
            map(
                identifier_string,
                move |name| {
                    let span = calculate_span(original_input, input);
                    Expr::Identifier(name, span)
                }
            ),
            // ブロック式 (UnitLiteralを返す場合があるので、リテラルの後に置く)
            |i| block_expr(i, original_input)
        ))
    )(input);

    result
}

// --- 古い paren_expr 関数は削除 ---

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

// Removed duplicate comparison function definition

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

/// 代入式をパース (右結合性)
/// LValue = Expression
fn assignment_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // 代入は右結合性を持つため、まず右辺をパースしようとするのではなく、
    // 左辺値候補 (logical_or) をパースし、次に '=' と右辺 (assignment_expr) をパースする
    let (input_after_left, left) = logical_or(input, original_input)?;


    // '=' が続くかチェック
    if let Ok((input_after_eq, _)) = ws_comments(tag("="))(input_after_left) {
        // '=' があれば、右辺の式を再帰的にパース (右結合性のため assignment_expr を呼ぶ)
        let (input_after_right, right) = assignment_expr(input_after_eq, original_input)?;

        // 左辺が代入可能かチェック (Identifier or MemberAccess)
        match &left {
            Expr::Identifier(..) | Expr::MemberAccess { .. } => {
                let span = calculate_span(original_input, input_after_right); // スパンは全体をカバー
                Ok((input_after_right, Expr::Assignment {
                    lvalue: Box::new(left),
                    rvalue: Box::new(right),
                    span,
                }))
            }
            _ => {
                // 代入不可能な式への代入エラー
                use nom::error::VerboseErrorKind;
                Err(nom::Err::Error(nom::error::VerboseError{ errors: vec![(input, VerboseErrorKind::Context("Invalid assignment target"))]}))
            }
        }
    } else {
        // '=' がなければ、左辺の式をそのまま返し、入力も左辺パース後のものを使う
        Ok((input_after_left, left))
    }
}

/// 比較演算をパース
pub fn comparison<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input_after_first, first) = term(input, original_input)?;

    let (input_after_rest, rest) = many0(
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
    )(input_after_first)?; // Use input_after_first

    let result = rest.into_iter().fold(first, |acc, (op, right)| {
        let span = calculate_span(original_input, input_after_rest); // Use input_after_rest for span end
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    });

    Ok((input_after_rest, result)) // Return input_after_rest
}

/// ガード節内の式をパース (代入式を含まない)
/// logical_or を開始点とする
pub fn parse_guard_expression<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    // 代入式を含まないため、logical_or から開始する
    logical_or(input, original_input)
}


/// if式をパース: if condition { then_branch } [elif condition { elif_branch }]* [else { else_branch }]?
/// すべての分岐本体はブロック式である必要がある
pub fn if_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize; // Span計算用

    // "if" condition { then_branch }
    let (input, _) = ws_comments(tag("if"))(input)?;
    let (input, condition) = expression(input, original_input)?;
    let (input, then_branch) = block_expr(input, original_input)?; // then節はブロック式必須

    // "[elif condition { elif_branch }]*"
    let (input, elif_branches) = many0(
        preceded(
            ws_comments(tag("elif")),
            pair(
                |i| expression(i, original_input), // elif の条件
                |i| block_expr(i, original_input)  // elif の本体 (ブロック式必須)
            )
        )
    )(input)?;

    // "[else { else_branch }]?"
    let (input, else_branch) = opt(
        preceded(
            ws_comments(tag("else")),
            |i| block_expr(i, original_input) // else節もブロック式必須
        )
    )(input)?;

    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize; // Span計算用
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);

    Ok((input, Expr::IfExpr {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        elif_branches, // パースした Vec<(Expr, Expr)> をそのまま渡す
        else_branch: else_branch.map(Box::new),
        span,
    }))
}

/// match式をパース
pub fn match_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("match"))(input)?;
    let (input, scrutinee) = expression(input, original_input)?;
    let (input, _) = ws_comments(char('{'))(input)?;

    let (input_after_cases, cases) = separated_list0(
        ws_comments(char(',')),
        |i| super::patterns::match_case(i, original_input, expression)
    )(input)?;

    let (input_after_opt_comma, _) = opt(ws_comments(char(',')))(input_after_cases)?;  // 末尾のカンマはオプション

    let (input, _) = cut(ws_comments(char('}')))(input_after_opt_comma)?; // Use input_after_opt_comma
    
    let span = calculate_span(original_input, input); // Use final input
    
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

/// with 式の束縛をパース: Identifier "=" Expression (":" TypeRef)?
fn parse_with_binding<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, WithBinding> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, alias) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, instance) = expression(input, original_input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;
    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);

    Ok((input, WithBinding {
        alias,
        instance,
        type_annotation,
        span,
    }))
}


/// with式をパース: with WithBinding ("," WithBinding)* BlockExpr
pub fn with_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, _) = keyword("with")(input)?;

    // 複数の束縛をパース
    let (input, bindings) = separated_list0(
        ws_comments(char(',')),
        |i| parse_with_binding(i, original_input)
    )(input)?;

    // 本体（ブロック式）
    let (input, body) = block_expr(input, original_input)?;

    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);

    Ok((input, Expr::WithExpr {
        bindings,
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

// parameter 関数は common.rs に移動

/// Effect パラメータをパース: effect identifier : Type
fn parse_effect_parameter<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, EffectParameter> {
    let (input, _) = keyword("effect")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char(':'))(input)?;
    let (input, effect_type) = parse_type(input, original_input)?;

    let span = calculate_span(original_input, input);

    Ok((input, EffectParameter {
        name,
        effect_type,
        span,
    }))
}

/// Effect パラメータリストをパース: ( EffectParam, ... )
fn parse_effect_parameter_list<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Vec<EffectParameter>> {
    delimited_list(
        '(',
        |i| parse_effect_parameter(i, original_input),
        ',',
        ')'
    )(input)
}

/// Implicit パラメータリストをパース: ( with Param, ... )
fn parse_implicit_parameter_list<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Vec<Parameter>> {
    delimited(
        ws_comments(char('(')),
        preceded(
            keyword("with"),
            separated_list0(
                ws_comments(char(',')),
                |i| parameter(i, original_input) // common::parameter を使用
            )
        ),
        cut(ws_comments(char(')')))
    )(input)
}

/// 関数式（旧ラムダ式）をパース: fn ParamList? EffectParamList? ImplicitParamList? (: ReturnType)? => Expression
pub fn function_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {

    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize; // Span計算用

    let (input, _) = keyword("fn")(input)?;

    // 通常のパラメータリスト (Option)
    let (input, parameters) = opt(
        delimited_list(
            '(',
            |i| parameter(i, original_input), // common::parameter を使用
            ',',
            ')'
        )
    )(input)?;

    // Effect パラメータリスト (Option)
    let (input, effect_parameters) = opt(|i| parse_effect_parameter_list(i, original_input))(input)?;

    // Implicit パラメータリスト (Option)
    let (input, implicit_parameters) = opt(|i| parse_implicit_parameter_list(i, original_input))(input)?;

    // 戻り値の型注釈 (Option)
    let (input, _return_type_annotation) = opt( // 変数は使わないがパースは行う
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;

    // '=>' トークン
    let (input, _) = cut(ws_comments(tag("=>")))(input)?;

    // 本体
    let (input, body) = cut(|i| expression(i, original_input))(input)?;

    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize; // Span計算用
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);

    Ok((input, Expr::FunctionExpr { // ASTバリアント名を変更
        parameters,
        effect_parameters,
        implicit_parameters,
        body: Box::new(body),
        span,
    }))
}

/// 式をパース
pub fn expression<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Expr> {

    // 特殊な式と論理OR演算（最も優先度の低い演算子）をパース
    // function_expr は 'fn' で始まるため、他のキーワードベースの式と同様に alt の先頭に配置
    alt((
        |i| function_expr(i, original_input), // function_expr に変更
        |i| if_expr(i, original_input),
        |i| match_expr(i, original_input),
        |i| bind_expr(i, original_input),
        |i| with_expr(i, original_input),
        // |i| lambda_expr(i, original_input), // 古い位置から削除
        |i| collection_literal(i, original_input),
        |i| collection_comprehension(i, original_input),
        // 代入式 (logical_or より優先度が低い)
        |i| assignment_expr(i, original_input),
        // logical_or は assignment_expr の中で最初に呼ばれるため、
        // ここで logical_or を直接呼ぶ必要はない。
        // ただし、代入式でない場合は logical_or 以下の式が直接パースされる必要があるため、
        // assignment_expr が '=' を見つけられなかった場合に logical_or の結果を返すようにする。
        // 現在の assignment_expr の実装はそうなっているので、これで良いはず。
    ))(input)
}
