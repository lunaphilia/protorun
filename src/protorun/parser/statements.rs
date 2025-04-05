// Protorun言語の文パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
};

// Stmt, Expr をインポート
use crate::protorun::ast::{Stmt, Expr};
use super::common::{ParseResult, ws_comments, calculate_span};
// use super::types::parse_type; // 不要になった
use super::expressions::expression;
// use super::common::{identifier_string, with_context}; // 不要になった
// use crate::protorun::ast::{Parameter, Decl}; // 不要になった

/// 文（現在は Return のみ）をパース
pub fn statement<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Stmt> {
    // alt から expression のパースを削除
    return_statement(input, original_input)
}

// let_statement 関数を削除
// var_statement 関数を削除

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

// function_declaration 関数を declarations.rs に移動したので削除
// parameter 関数を declarations.rs に移動したので削除

/// トップレベル要素（宣言または式）
#[derive(Debug, Clone, PartialEq)]
enum TopLevelItem {
    Declaration(crate::protorun::ast::Decl),
    TypeDeclaration(crate::protorun::ast::TypeDecl),
    TraitDeclaration(crate::protorun::ast::TraitDecl),
    ImplDeclaration(crate::protorun::ast::ImplDecl),
    Module(crate::protorun::ast::Module),
    Expression(Expr),
}

/// プログラム全体をパース (Program ::= (Declaration | Expression)*)
pub fn program<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, crate::protorun::ast::Program> {
    use nom::character::complete::multispace0;
    use nom::multi::many0;
    use nom::sequence::terminated;
    use super::declarations::{parse_declaration, parse_type_declaration, parse_trait_declaration, parse_impl_declaration};
    use super::modules::parse_module;

    let (input, _) = multispace0(input)?;

    // トップレベル要素（宣言または式）をパース
    let (input, items) = many0(terminated(
        alt((
            // 各種宣言パーサー
            map(|i| parse_declaration(i, original_input), TopLevelItem::Declaration),
            map(|i| parse_type_declaration(i, original_input), TopLevelItem::TypeDeclaration),
            map(|i| parse_trait_declaration(i, original_input), TopLevelItem::TraitDeclaration),
            map(|i| parse_impl_declaration(i, original_input), TopLevelItem::ImplDeclaration),
            map(|i| parse_module(i, original_input), TopLevelItem::Module),
            // 式パーサー
            map(|i| expression(i, original_input), TopLevelItem::Expression),
        )),
        // 各トップレベル要素の後には空白が続くことを想定。
        // Protorun では文の区切りにセミコロンは不要で、改行や空白で区切られるため、
        // terminated(..., multispace0) で要素間の区切りを処理する。
        multispace0
    ))(input)?;

    let (input, _) = multispace0(input)?; // 末尾の空白

    // パース結果を Program 構造体に振り分ける
    let mut modules = Vec::new();
    let mut declarations = Vec::new();
    let mut type_declarations = Vec::new();
    let mut trait_declarations = Vec::new();
    let mut impl_declarations = Vec::new();
    // let mut statements = Vec::new(); // 削除
    let mut expressions = Vec::new(); // 追加

    for item in items {
        match item {
            TopLevelItem::Declaration(decl) => declarations.push(decl),
            TopLevelItem::TypeDeclaration(decl) => type_declarations.push(decl),
            TopLevelItem::TraitDeclaration(decl) => trait_declarations.push(decl),
            TopLevelItem::ImplDeclaration(decl) => impl_declarations.push(decl),
            TopLevelItem::Module(module) => modules.push(module),
            TopLevelItem::Expression(expr) => expressions.push(expr), // expressions に追加
        }
    }

    Ok((input, crate::protorun::ast::Program {
        modules,
        declarations,
        type_declarations,
        trait_declarations,
        impl_declarations,
        expressions, // statements を expressions に変更
    }))
}
