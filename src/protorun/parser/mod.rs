// Protorun言語の構文解析器 - nomパーサーコンビネータ版

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1, none_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{context, ErrorKind, VerboseError},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Finish, IResult,
};

use super::ast::{
    BinaryOperator, Decl, Expr, Parameter, Program, Span, Stmt, Type, UnaryOperator,
};
use super::error::{Error, Result};

/// パーサー
pub struct Parser {
    /// ファイル名
    filename: Option<String>,
}

impl Parser {
    /// 新しいパーサーを作成
    pub fn new(filename: Option<String>) -> Self {
        Self { filename }
    }

    // プログラム全体をパース
    pub fn parse_program(&mut self, input: &str) -> Result<Program> {
        match program(input).finish() {
            Ok((_, program)) => Ok(program),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }

    // 式をパース
    pub fn parse_expression(&mut self, input: &str) -> Result<Expr> {
        match expression(input).finish() {
            Ok((_, expr)) => Ok(expr),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
    
    // 型をパース
    pub fn parse_type(&mut self, input: &str) -> Result<super::ast::Type> {
        match type_parser(input).finish() {
            Ok((_, ty)) => Ok(ty),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
}

// パーサーの結果型
type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

// 構文エラーをProtorunのエラーに変換
fn to_syntax_error<'a>(input: &'a str, error: VerboseError<&'a str>, filename: Option<String>) -> Error {
    // 簡単なエラーメッセージの生成
    let message = if error.errors.is_empty() {
        "構文解析エラー".to_string()
    } else {
        let (input_slice, kind) = &error.errors[0];
        match kind {
            nom::error::VerboseErrorKind::Nom(ErrorKind::Tag) => format!("期待されるキーワードが見つかりません: '{}'", input_slice),
            nom::error::VerboseErrorKind::Nom(ErrorKind::Char) => format!("期待される文字が見つかりません: '{}'", input_slice),
            nom::error::VerboseErrorKind::Nom(ErrorKind::Eof) => "式が期待されます".to_string(),
            nom::error::VerboseErrorKind::Context(_) => "式が期待されます".to_string(),
            _ => format!("構文解析エラー: {:?}", kind),
        }
    };

    // エラーの位置情報
    // 正確な位置情報を取得するのは難しいため、おおよその位置を設定
    let pos = input.len().saturating_sub(input.trim_start().len());
    let span = Span {
        start: pos,
        end: pos + 1,
        line: 1 + input[..pos].chars().filter(|&c| c == '\n').count(),
        column: 1 + input[..pos].chars().rev().take_while(|&c| c != '\n').count(),
    };

    Error::syntax(message, Some(span), filename)
}

// 行コメントをスキップ
fn skip_comment(input: &str) -> ParseResult<&str> {
    preceded(
        tag("//"),
        terminated(
            take_while1(|c| c != '\n'),
            alt((value((), char('\n')), value((), nom::combinator::eof)))
        )
    )(input)
}

// 空白とコメントをスキップ（コメント対応版）
fn ws_comments<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> ParseResult<'a, O>
where
    F: FnMut(&'a str) -> ParseResult<'a, O>,
{
    delimited(
        many0(alt((
            value((), multispace1),
            value((), skip_comment),
        ))),
        inner,
        many0(alt((
            value((), multispace1),
            value((), skip_comment),
        )))
    )
}

// 識別子をパース
fn identifier(input: &str) -> ParseResult<&str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

// 識別子文字列をパース
fn identifier_string(input: &str) -> ParseResult<String> {
    map(identifier, |s: &str| s.to_string())(input)
}

// 整数リテラルをパース
fn int_literal(input: &str) -> ParseResult<i64> {
    map_res(
        recognize(
            pair(
                opt(char('-')),
                digit1
            )
        ),
        |s: &str| s.parse::<i64>()
    )(input)
}

// 浮動小数点リテラルをパース
fn float_literal(input: &str) -> ParseResult<f64> {
    map_res(
        recognize(
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

// 文字列リテラルをパース
fn string_literal(input: &str) -> ParseResult<String> {
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

// 真偽値リテラルをパース
fn bool_literal(input: &str) -> ParseResult<bool> {
    alt((
        value(true, tag("true")),
        value(false, tag("false"))
    ))(input)
}

// 基本式をパース
fn primary(input: &str) -> ParseResult<Expr> {
    ws_comments(
        alt((
            // 整数リテラル
            map(int_literal, |value| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::IntLiteral(value, span)
            }),
            // 浮動小数点リテラル
            map(float_literal, |value| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::FloatLiteral(value, span)
            }),
            // 文字列リテラル
            map(string_literal, |value| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::StringLiteral(value, span)
            }),
            // 真偽値リテラル
            map(bool_literal, |value| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::BoolLiteral(value, span)
            }),
            // 括弧式
            map(
                delimited(
                    char('('),
                    ws_comments(expression),
                    cut(char(')'))
                ),
                |expr| {
                    let span = Span {
                        start: 0,
                        end: 0,
                        line: 0,
                        column: 0,
                    };
                    Expr::ParenExpr(Box::new(expr), span)
                }
            ),
            // 識別子
            map(identifier_string, |name| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::Identifier(name, span)
            }),
            // ブロック式
            map(
                delimited(
                    char('{'),
                    block_contents,
                    cut(char('}'))
                ),
                |(_statements, expr)| {
                    let span = Span {
                        start: 0,
                        end: 0,
                        line: 0,
                        column: 0,
                    };
                    match expr {
                        Some(e) => e,
                        None => Expr::UnitLiteral(span)
                    }
                }
            )
        ))
    )(input)
}

// ブロックの内容をパース
fn block_contents(input: &str) -> ParseResult<(Vec<Stmt>, Option<Expr>)> {
    let (input, statements) = many0(terminated(statement, ws_comments(char(';'))))(input)?;
    let (input, expr) = opt(expression)(input)?;
    Ok((input, (statements, expr)))
}

// 関数呼び出しをパース
fn function_call(input: &str) -> ParseResult<Expr> {
    let (input, func) = primary(input)?;
    
    let (input, args_opt) = opt(
        delimited(
            ws_comments(char('(')),
            separated_list0(
                ws_comments(char(',')),
                expression
            ),
            cut(ws_comments(char(')')))
        )
    )(input)?;
    
    match args_opt {
        Some(args) => {
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            Ok((input, Expr::FunctionCall {
                function: Box::new(func),
                arguments: args,
                span,
            }))
        },
        None => Ok((input, func))
    }
}

// 単項演算をパース
fn unary(input: &str) -> ParseResult<Expr> {
    alt((
        map(
            pair(
                ws_comments(char('-')),
                unary
            ),
            |(_, expr)| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
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
                unary
            ),
            |(_, expr)| {
                let span = Span {
                    start: 0,
                    end: 0,
                    line: 0,
                    column: 0,
                };
                Expr::UnaryOp {
                    operator: UnaryOperator::Not,
                    expr: Box::new(expr),
                    span,
                }
            }
        ),
        function_call
    ))(input)
}

// 因子をパース（乗除算）
fn factor(input: &str) -> ParseResult<Expr> {
    let (input, first) = unary(input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Mul, char('*')),
                value(BinaryOperator::Div, char('/')),
                value(BinaryOperator::Mod, char('%'))
            ))),
            unary
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

// 項をパース（加減算）
fn term(input: &str) -> ParseResult<Expr> {
    let (input, first) = factor(input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Add, char('+')),
                value(BinaryOperator::Sub, char('-'))
            ))),
            factor
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

// 比較演算をパース
fn comparison(input: &str) -> ParseResult<Expr> {
    let (input, first) = term(input)?;
    
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
            term
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

// 等価演算をパース
fn equality(input: &str) -> ParseResult<Expr> {
    let (input, first) = comparison(input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(alt((
                value(BinaryOperator::Eq, tag("==")),
                value(BinaryOperator::Neq, tag("!="))
            ))),
            comparison
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (op, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: op,
            right: Box::new(right),
            span,
        }
    })))
}

// 論理AND演算をパース
fn logical_and(input: &str) -> ParseResult<Expr> {
    let (input, first) = equality(input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("&&")),
            equality
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (_, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: BinaryOperator::And,
            right: Box::new(right),
            span,
        }
    })))
}

// 論理OR演算をパース
fn logical_or(input: &str) -> ParseResult<Expr> {
    let (input, first) = logical_and(input)?;
    
    let (input, rest) = many0(
        pair(
            ws_comments(tag("||")),
            logical_and
        )
    )(input)?;
    
    Ok((input, rest.into_iter().fold(first, |acc, (_, right)| {
        let span = Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        };
        Expr::BinaryOp {
            left: Box::new(acc),
            operator: BinaryOperator::Or,
            right: Box::new(right),
            span,
        }
    })))
}

// 式をパース
fn expression(input: &str) -> ParseResult<Expr> {
    logical_or(input)
}

// 単純型をパース
fn simple_type(input: &str) -> ParseResult<Type> {
    map(
        ws_comments(identifier_string),
        |name| {
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            Type::Simple { name, span }
        }
    )(input)
}

// ジェネリック型引数をパース
fn generic_args(input: &str) -> ParseResult<Vec<Type>> {
    delimited(
        ws_comments(char('<')),
        separated_list0(
            ws_comments(char(',')),
            type_parser
        ),
        cut(ws_comments(char('>')))
    )(input)
}

// ジェネリック型をパース
fn generic_type(input: &str) -> ParseResult<Type> {
    let (input, base_name) = ws_comments(identifier_string)(input)?;
    let (input, args_opt) = opt(generic_args)(input)?;
    
    match args_opt {
        Some(args) if !args.is_empty() => {
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            Ok((input, Type::Generic {
                base_type: base_name,
                type_arguments: args,
                span,
            }))
        },
        _ => {
            // ジェネリック引数がない場合は単純型として扱う
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            Ok((input, Type::Simple { name: base_name, span }))
        }
    }
}

// 配列型をパース
fn array_type(input: &str) -> ParseResult<Type> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, element_type) = type_parser(input)?;
    let (input, _) = cut(ws_comments(char(']')))(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Type::Array {
        element_type: Box::new(element_type),
        span,
    }))
}

// タプル型をパース
fn tuple_type(input: &str) -> ParseResult<Type> {
    let (input, _) = ws_comments(char('('))(input)?;
    let (input, first_type) = type_parser(input)?;
    
    // カンマがある場合はタプル型、ない場合は括弧で囲まれた型
    let (input, rest) = opt(
        preceded(
            ws_comments(char(',')),
            separated_list0(
                ws_comments(char(',')),
                type_parser
            )
        )
    )(input)?;
    
    let (input, _) = cut(ws_comments(char(')')))(input)?;
    
    match rest {
        Some(mut types) => {
            // タプル型
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            
            let mut element_types = vec![first_type];
            element_types.append(&mut types);
            
            Ok((input, Type::Tuple {
                element_types,
                span,
            }))
        },
        None => {
            // 括弧で囲まれた型
            Ok((input, first_type))
        }
    }
}

// 効果型をパース
fn effect_type(input: &str) -> ParseResult<Type> {
    preceded(
        ws_comments(tag("&")),
        type_parser
    )(input)
}

// 関数型をパース
fn function_type(input: &str) -> ParseResult<Type> {
    let (input, _) = ws_comments(char('('))(input)?;
    
    // カンマで区切られた型のリストをパース
    let (input, first_type_opt) = opt(type_parser)(input)?;
    
    let (input, params) = match first_type_opt {
        Some(first_type) => {
            let (input, rest_types) = many0(
                preceded(
                    ws_comments(char(',')),
                    type_parser
                )
            )(input)?;
            
            let mut params = vec![first_type];
            params.extend(rest_types);
            (input, params)
        },
        None => (input, vec![])
    };
    
    let (input, _) = cut(ws_comments(char(')')))(input)?;
    
    let (input, _) = ws_comments(tag("->"))(input)?;
    let (input, return_type) = type_parser(input)?;
    
    // オプションの効果型
    let (input, effect) = opt(effect_type)(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    let function_type = Type::Function {
        parameters: params,
        return_type: Box::new(return_type),
        span: span.clone(),
    };
    
    match effect {
        Some(effect_type) => {
            Ok((input, Type::WithEffect {
                base_type: Box::new(function_type),
                effect_type: Box::new(effect_type),
                span,
            }))
        },
        None => Ok((input, function_type))
    }
}

// 参照型をパース
fn reference_type(input: &str) -> ParseResult<Type> {
    let (input, is_mutable) = alt((
        value(true, preceded(ws_comments(char('&')), ws_comments(tag("mut")))),
        value(false, ws_comments(char('&')))
    ))(input)?;
    
    let (input, referenced_type) = type_parser(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Type::Reference {
        is_mutable,
        referenced_type: Box::new(referenced_type),
        span,
    }))
}

// 所有権型をパース
fn owned_type(input: &str) -> ParseResult<Type> {
    let (input, _) = ws_comments(tag("own"))(input)?;
    let (input, owned_type) = type_parser(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Type::Owned {
        owned_type: Box::new(owned_type),
        span,
    }))
}

// 型をパース（統合版）
fn type_parser(input: &str) -> ParseResult<Type> {
    alt((
        owned_type,
        reference_type,
        function_type,
        array_type,
        tuple_type,
        generic_type,
        simple_type  // simple_typeを追加
    ))(input)
}

// パラメータをパース
fn parameter(input: &str) -> ParseResult<Parameter> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            type_parser
        )
    )(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Parameter {
        name,
        type_annotation,
        span,
    }))
}

// let文をパース
fn let_statement(input: &str) -> ParseResult<Stmt> {
    let (input, _) = ws_comments(tag("let"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            type_parser
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    // ここにコンテキストを追加
    let (input, value) = context("expression", cut(expression))(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Stmt::Let {
        name,
        type_annotation,
        value,
        span,
    }))
}

// 文をパース
fn statement(input: &str) -> ParseResult<Stmt> {
    alt((
        let_statement,
        map(expression, |expr| {
            let span = Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            };
            Stmt::Expr { expr, span }
        })
    ))(input)
}

// 関数宣言をパース
fn function_declaration(input: &str) -> ParseResult<Decl> {
    let (input, _) = ws_comments(tag("fn"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, parameters) = delimited(
        ws_comments(char('(')),
        separated_list0(
            ws_comments(char(',')),
            parameter
        ),
        cut(ws_comments(char(')')))
    )(input)?;
    let (input, return_type) = opt(
        preceded(
            ws_comments(char(':')),
            type_parser
        )
    )(input)?;
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = cut(expression)(input)?;
    let (input, _) = opt(ws_comments(char(';')))(input)?;
    
    let span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };
    
    Ok((input, Decl::Function {
        name,
        parameters,
        return_type,
        body,
        span,
    }))
}

// プログラム全体をパース
fn program(input: &str) -> ParseResult<Program> {
    let (input, _) = multispace0(input)?;
    let (input, declarations) = many0(function_declaration)(input)?;
    let (input, statements) = many0(
        terminated(
            statement,
            ws_comments(char(';'))
        )
    )(input)?;
    let (input, _) = multispace0(input)?;
    
    Ok((input, Program {
        declarations,
        statements,
    }))
}

#[cfg(test)]
mod tests;
