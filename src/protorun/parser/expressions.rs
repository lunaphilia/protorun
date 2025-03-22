// Protorun言語の式パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{cut, map, opt, value},
    error::{VerboseError, ParseError},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use crate::protorun::ast::{Expr, Span, BinaryOperator, UnaryOperator, HandlerSpec, ComprehensionKind};
use crate::protorun::symbol::ScopeKind;
use super::common::{ParseResult, ParserContext, ws_comments, identifier_string, with_context, delimited_list};
use super::literals::{int_literal_expr, float_literal_expr, string_literal_expr, bool_literal_expr, unit_literal_expr};
use super::patterns::{pattern, match_case};
use super::types::parse_type;

/// 括弧式をパース
pub fn paren_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    // ラムダ式のパターンに一致する場合のチェックは不要
    // lambda_exprがparen_exprよりも先に試されるため
    
    let (input, _) = char('(')(input)?;
    let (input, _) = multispace0(input)?;
    
    // 空の括弧 -> ユニットリテラル
    if let Ok((_, _)) = char::<&str, VerboseError<&str>>(')')(&input) {
        let (input, _) = char(')')(input)?;
        let span = ctx.calculate_span(input);
        return Ok((input, Expr::UnitLiteral(span)));
    }
    
    // 括弧内の式をパース
    let (input, expr) = ws_comments(|i| expression(i, ctx))(input)?;
    let (input, _) = cut(ws_comments(char(')')))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::ParenExpr(Box::new(expr), span)))
}

/// 基本式をパース
pub fn primary<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let result = ws_comments(
        alt((
            // 括弧式
            |i| paren_expr(i, ctx),
            // 整数リテラル
            |i| int_literal_expr(i, ctx),
            // 浮動小数点リテラル
            |i| float_literal_expr(i, ctx),
            // 文字列リテラル
            |i| string_literal_expr(i, ctx),
            // 真偽値リテラル
            |i| bool_literal_expr(i, ctx),
            // ユニットリテラル
            |i| unit_literal_expr(i, ctx),
            // 識別子
            map(
                identifier_string,
                move |name| {
                    let span = ctx.calculate_span(input);
                    
                    // シンボルテーブルで名前解決（エラーは報告するが、パースは続行）
                    if let Some(_) = ctx.lookup_symbol(&name) {
                        // シンボルの使用をマーク
                        let _ = ctx.mark_symbol_used(&name);
                    }
                    
                    Expr::Identifier(name, span)
                }
            ),
            // ブロック式
            |i| block_expr(i, ctx)
        ))
    )(input);
    
    result
}

/// ブロック式をパース
pub fn block_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('{'))(input)?;
    
    // ブロックスコープを開始
    ctx.enter_scope(ScopeKind::Block);
    
    let (input, (statements, expr)) = block_contents(input, ctx)?;
    let (input, _) = cut(ws_comments(char('}')))(input)?;
    
    let span = ctx.calculate_span(input);
    
    // 最後の式がある場合はそれを返し、なければUnitLiteralを返す
    let result = match expr {
        Some(e) => e,
        None => Expr::UnitLiteral(span),
    };
    
    // ブロックスコープを終了
    ctx.exit_scope();
    
    Ok((input, result))
}

/// ブロックの内容をパース
pub fn block_contents<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, (Vec<crate::protorun::ast::Stmt>, Option<Expr>)> {
    use super::statements::statement;
    
    let (input, statements) = many0(
        terminated(
            |i| statement(i, ctx),
            ws_comments(char(';'))
        )
    )(input)?;
    let (input, expr) = opt(|i| expression(i, ctx))(input)?;
    
    Ok((input, (statements, expr)))
}

/// 後置式（関数呼び出しとメンバーアクセス）をパース
pub fn postfix<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (mut current_input, mut expr) = primary(input, ctx)?;
    
    // 関数呼び出しとメンバーアクセスを繰り返しパース
    loop {
        // 関数呼び出し
        if let Ok((new_input, args)) = delimited(
            ws_comments(char('(')),
            separated_list0(
                ws_comments(char(',')),
                |i| expression(i, ctx)
            ),
            cut(ws_comments(char(')')))
        )(current_input) {
            let span = ctx.calculate_span(new_input);
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
                let span = ctx.calculate_span(new_input);
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
pub fn unary<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    alt((
        map(
            pair(
                ws_comments(char('-')),
                |i| unary(i, ctx)
            ),
            move |(_, expr)| {
                let span = ctx.calculate_span(input);
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
                |i| unary(i, ctx)
            ),
            move |(_, expr)| {
                let span = ctx.calculate_span(input);
                Expr::UnaryOp {
                    operator: UnaryOperator::Not,
                    expr: Box::new(expr),
                    span,
                }
            }
        ),
        |i| postfix(i, ctx)
    ))(input)
}

/// 因子をパース（乗除算）
pub fn factor<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = unary(input, ctx)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Mul, char('*')),
                value(BinaryOperator::Div, char('/')),
                value(BinaryOperator::Mod, char('%'))
            ))),
            |i| unary(i, ctx)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (op, right)| {
        let span = ctx.calculate_span(input);
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
pub fn term<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = factor(input, ctx)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Add, char('+')),
                value(BinaryOperator::Sub, char('-'))
            ))),
            |i| factor(i, ctx)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (op, right)| {
        let span = ctx.calculate_span(input);
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
pub fn comparison<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = term(input, ctx)?;
    
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
            |i| term(i, ctx)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = ctx.calculate_span(input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

/// 等価演算をパース
pub fn equality<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = comparison(input, ctx)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Eq, tag("==")),
                value(BinaryOperator::Neq, tag("!="))
            ))),
            |i| comparison(i, ctx)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = ctx.calculate_span(input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

/// 論理AND演算をパース
pub fn logical_and<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = equality(input, ctx)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("&&")),
            |i| equality(i, ctx)
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (_, right)| {
        let span = ctx.calculate_span(input);
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: BinaryOperator::And,
            right: Box::new(right),
            span,
        }
    })))
}

/// 論理OR演算をパース
pub fn logical_or<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, first) = logical_and(input, ctx)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("||")),
            |i| logical_and(i, ctx)
        )
    )(input)?;
    
    let result = rest.into_iter().fold(first, |acc, (_, right)| {
        let span = ctx.calculate_span(input);
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
pub fn if_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("if"))(input)?;
    let (input, condition) = expression(input, ctx)?;
    let (input, then_branch) = block_expr(input, ctx)?;
    let (input, else_branch) = opt(
        preceded(
            ws_comments(tag("else")),
            alt((
                |i| if_expr(i, ctx),
                |i| block_expr(i, ctx)
            ))
        )
    )(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::IfExpr {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: else_branch.map(Box::new),
        span,
    }))
}

/// match式をパース
pub fn match_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("match"))(input)?;
    let (input, scrutinee) = expression(input, ctx)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, cases) = separated_list0(
        ws_comments(char(',')),
        |i| super::patterns::match_case(i, ctx, expression)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = cut(ws_comments(char('}')))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::MatchExpr {
        scrutinee: Box::new(scrutinee),
        cases,
        span,
    }))
}

/// リスト内包表記をパース
pub fn list_comprehension<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, output_expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, ctx)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, ctx)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char(']'))(input)?;
    
    let span = ctx.calculate_span(input);
    
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
pub fn map_comprehension<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, key_expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(tag("->"))(input)?;
    let (input, value_expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, ctx)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, ctx)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    // マップ内包表記は、キーと値のペアを出力する特殊なケース
    // 内部的には、タプル式を出力するリスト内包表記として扱う
    let span = ctx.calculate_span(input);
    
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
pub fn set_comprehension<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("#{"))(input)?;
    let (input, output_expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(tag("for"))(input)?;
    let (input, pat) = pattern(input, ctx)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, input_expr) = expression(input, ctx)?;
    let (input, condition) = opt(
        preceded(
            ws_comments(tag("if")),
            |i| expression(i, ctx)
        )
    )(input)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = ctx.calculate_span(input);
    
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
pub fn collection_comprehension<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    alt((
        |i| list_comprehension(i, ctx),
        |i| map_comprehension(i, ctx),
        |i| set_comprehension(i, ctx)
    ))(input)
}

/// bind式のバインド文をパース
pub fn bind_statement<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, (crate::protorun::ast::Pattern, Expr)> {
    let (input, pat) = pattern(input, ctx)?;
    let (input, _) = ws_comments(tag("<-"))(input)?;
    let (input, expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(char(';'))(input)?;
    
    Ok((input, (pat, expr)))
}

/// bind式をパース
pub fn bind_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("bind"))(input)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    
    let (input, bindings) = many0(|i| bind_statement(i, ctx))(input)?;
    
    let (input, final_expr) = expression(input, ctx)?;
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::BindExpr {
        bindings,
        final_expr: Box::new(final_expr),
        span,
    }))
}

/// with式をパース
pub fn with_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("with"))(input)?;
    
    // ハンドラ指定（式または型）
    let (input, handler) = alt((
        // 型としてのハンドラ
        map(|i| parse_type(i, ctx), HandlerSpec::Type),
        // 式としてのハンドラ
        map(|i| logical_or(i, ctx), |expr| HandlerSpec::Expr(Box::new(expr)))
    ))(input)?;
    
    // オプションの効果型
    let (input, effect_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, ctx)
        )
    )(input)?;
    
    // 本体（ブロック式）
    let (input, body) = block_expr(input, ctx)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::WithExpr {
        handler,
        effect_type,
        body: Box::new(body),
        span,
    }))
}

/// リストリテラルをパース
pub fn list_literal<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, elements) = separated_list0(
        ws_comments(char(',')),
        |i| expression(i, ctx)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char(']'))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::ListLiteral {
        elements,
        span,
    }))
}

/// マップリテラルをパース
pub fn map_literal<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(char('{'))(input)?;
    let (input, entries) = separated_list0(
        ws_comments(char(',')),
        |i| {
            let (i, key) = expression(i, ctx)?;
            let (i, _) = ws_comments(tag("->"))(i)?;
            let (i, value) = expression(i, ctx)?;
            Ok((i, (key, value)))
        }
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::MapLiteral {
        entries,
        span,
    }))
}

/// セットリテラルをパース
pub fn set_literal<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    let (input, _) = ws_comments(tag("#{"))(input)?;
    let (input, elements) = separated_list0(
        ws_comments(char(',')),
        |i| expression(i, ctx)
    )(input)?;
    let (input, _) = opt(ws_comments(char(',')))(input)?;  // 末尾のカンマはオプション
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = ctx.calculate_span(input);
    
    Ok((input, Expr::SetLiteral {
        elements,
        span,
    }))
}

/// コレクションリテラルをパース（統合版）
pub fn collection_literal<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    alt((
        |i| list_literal(i, ctx),
        |i| map_literal(i, ctx),
        |i| set_literal(i, ctx)
    ))(input)
}

/// ラムダ式をパース
pub fn lambda_expr<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    // 入力が既に'('で始まっているかどうかを確認
    if input.starts_with('(') {
        // 先読みでラムダ式かどうかを確認
        let (params_input, _) = char('(')(input)?;
        let params_result = separated_list0(
            ws_comments(char(',')),
            |i| super::statements::parameter(i, ctx)
        )(params_input);
        
        if let Ok((params_rest, _)) = params_result {
            let close_paren_result = ws_comments(char(')'))(params_rest);
            if let Ok((after_paren, _)) = close_paren_result {
                let arrow_result = ws_comments(tag("=>"))(after_paren);
                if arrow_result.is_ok() {
                    // ラムダ式と確認できたので、パースを続行
                    
                    // 通常のパラメータリストのパース
                    let (input, parameters) = delimited_list(
                        '(',
                        |i| super::statements::parameter(i, ctx),
                        ',',
                        ')'
                    )(input)?;
                    
                    // "=>"トークンをパース
                    let (input, _) = ws_comments(tag("=>"))(input)?;
                    
                    // 本体の式をパース
                    let (input, body) = expression(input, ctx)?;
                    
                    let span = ctx.calculate_span(input);
                    
                    return Ok((input, Expr::LambdaExpr {
                        parameters,
                        body: Box::new(body),
                        span,
                    }));
                }
            }
        }
        
        // ラムダ式ではないと判断
        return Err(nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Nom(nom::error::ErrorKind::Tag))] }));
    } else {
        // 既に'('が消費されている場合は、パラメータリストを直接パース
        // パラメータリストをパース（'('は既に消費されている）
        let (input, params) = separated_list0(
            ws_comments(char(',')),
            |i| super::statements::parameter(i, ctx)
        )(input)?;
        let (input, _) = ws_comments(char(')'))(input)?;
        
        // "=>"トークンをパース
        let (input, _) = ws_comments(tag("=>"))(input)?;
        
        // 本体の式をパース
        let (input, body) = expression(input, ctx)?;
        
        let span = ctx.calculate_span(input);
        
        Ok((input, Expr::LambdaExpr {
            parameters: params,
            body: Box::new(body),
            span,
        }))
    }
}

/// 式をパース
pub fn expression<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, Expr> {
    // 宣言的なアプローチでパーサーを組み合わせる
    let result = alt((
        |i| lambda_expr(i, ctx),
        |i| if_expr(i, ctx),
        |i| match_expr(i, ctx),
        |i| list_comprehension(i, ctx),
        |i| list_literal(i, ctx),
        |i| map_comprehension(i, ctx),
        |i| map_literal(i, ctx),
        |i| set_comprehension(i, ctx),
        |i| set_literal(i, ctx),
        |i| bind_expr(i, ctx),
        |i| with_expr(i, ctx),
        |i| block_expr(i, ctx),
        |i| logical_or(i, ctx),
        |i| paren_expr(i, ctx)
    ))(input);
    
    result
}
