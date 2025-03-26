// Protorun言語のモジュールパーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{cut, map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::protorun::ast::{Module, ExportDecl, ImportDecl, ImportItem, Span, Decl};
use crate::protorun::symbol::{Symbol, SymbolKind, ScopeKind};
use super::common::{ParseResult, ParserContext, ws_comments, identifier_string, delimited_list};
use super::declarations::{parse_type_declaration, parse_trait_declaration, parse_impl_declaration};
use super::statements::{statement, function_declaration};

/// インポート種別
enum ImportType {
    Selective(Vec<ImportItem>, String),
    Module(String, String),
}

/// エクスポート宣言のパース
pub fn parse_export<'a>(input: &'a str, ctx: &mut ParserContext<'a>) -> ParseResult<'a, (ExportDecl, Option<Decl>)> {
    let (input, _) = ws_comments(tag("export"))(input)?;
    
    // 関数宣言のエクスポート
    let (input, decl) = opt(|i| function_declaration(i, ctx))(input)?;
    
    if let Some(decl) = decl {
        let span = ctx.calculate_span(input);
        match &decl {
            Decl::Function { name, .. } => {
                return Ok((input, (ExportDecl::Single { name: name.clone(), span }, Some(decl))));
            }
            _ => unreachable!(),
        }
    }
    
    // グループエクスポートまたは個別エクスポート
    let (input, export) = alt((
        // グループエクスポート
        map(
            preceded(
                ws_comments(char('{')),
                cut(terminated(
                    many0(terminated(
                        ws_comments(identifier_string),
                        opt(ws_comments(char(',')))
                    )),
                    ws_comments(char('}'))
                ))
            ),
            |names| {
                let span = ctx.calculate_span(input);
                (ExportDecl::Group { names, span }, None)
            }
        ),
        // 個別エクスポート（識別子のみ）
        map(
            ws_comments(identifier_string),
            |name| {
                let span = ctx.calculate_span(input);
                (ExportDecl::Single { name, span }, None)
            }
        )
    ))(input)?;
    
    Ok((input, export))
}

/// インポートアイテムのパース
fn parse_import_item<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, ImportItem> {
    println!("parse_import_item: 開始 input='{}'", input.trim());
    let (input, name) = ws_comments(identifier_string)(input)?;
    println!("parse_import_item: 名前をパース name='{}'", name);
    
    // asキーワードの後のスペースを必須にしない
    let (input, alias) = opt(
        preceded(
            ws_comments(tag("as")),
            ws_comments(identifier_string)
        )
    )(input)?;
    
    println!("parse_import_item: エイリアスをパース alias={:?}", alias);
    
    let span = ctx.calculate_span(input);
    
    let result = ImportItem {
        name,
        alias,
        span,
    };
    
    println!("parse_import_item: 終了 result={{ name: {}, alias: {:?} }}", result.name, result.alias);
    Ok((input, result))
}

/// インポート宣言のパース
pub fn parse_import<'a>(input: &'a str, ctx: &ParserContext<'a>) -> ParseResult<'a, ImportDecl> {
    println!("parse_import: 開始 input='{}'", input.trim());
    let (input, _) = ws_comments(tag("import"))(input)?;
    
    // 選択的インポートまたはモジュール全体のインポートをパース
    let (input, import_type) = alt((
        // 選択的インポート
        map(
            tuple((
                preceded(
                    ws_comments(tag("{")),
                    cut(terminated(
                        many0(terminated(
                            |i: &'a str| -> ParseResult<'a, ImportItem> {
                                println!("parse_import: インポートアイテムのパース開始 i='{}'", i.trim());
                                let result = parse_import_item(i, ctx);
                                match &result {
                                    Ok((_, item)) => println!("parse_import: インポートアイテムのパース成功 name={}, alias={:?}", item.name, item.alias),
                                    Err(e) => println!("parse_import: インポートアイテムのパース失敗 error={:?}", e),
                                }
                                result
                            },
                            opt(ws_comments(char(',')))
                        )),
                        ws_comments(char('}'))
                    ))
                ),
                preceded(
                    ws_comments(tag("from")),
                    delimited(
                        ws_comments(char('"')),
                        identifier_string,
                        ws_comments(char('"'))
                    )
                )
            )),
            |(imports, module_path)| {
                println!("parse_import: 選択的インポート module_path={}, imports.len()={}", module_path, imports.len());
                for (i, item) in imports.iter().enumerate() {
                    println!("parse_import:   imports[{}]: name={}, alias={:?}", i, item.name, item.alias);
                }
                ImportType::Selective(imports, module_path)
            }
        ),
        // モジュール全体のインポート
        map(
            tuple((
                delimited(
                    ws_comments(char('"')),
                    identifier_string,
                    ws_comments(char('"'))
                ),
                preceded(
                    ws_comments(tag("as")),
                    ws_comments(identifier_string)
                )
            )),
            |(module_path, alias)| {
                println!("parse_import: モジュール全体のインポート module_path={}, alias={}", module_path, alias);
                ImportType::Module(module_path, alias)
            }
        )
    ))(input)?;

    let span = ctx.calculate_span(input);
    let import = match import_type {
        ImportType::Selective(imports, module_path) => {
            println!("parse_import: ImportDecl::Selective 作成 module_path={}, imports.len()={}", module_path, imports.len());
            ImportDecl::Selective {
                module_path,
                imports,
                span,
            }
        },
        ImportType::Module(module_path, alias) => {
            println!("parse_import: ImportDecl::Module 作成 module_path={}, alias={}", module_path, alias);
            ImportDecl::Module {
                module_path,
                alias,
                span,
            }
        },
    };
    
    println!("parse_import: 終了");
    Ok((input, import))
}

/// モジュール宣言のパース
pub fn parse_module<'a>(input: &'a str, ctx: &mut ParserContext<'a>) -> ParseResult<'a, Module> {
    let (input, _) = ws_comments(tag("module"))(input)?;
    let (input, path) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    
    // モジュールスコープを開始
    ctx.enter_scope(ScopeKind::Module);
    
    // エクスポート宣言をパース
    let (input, export_results) = many0(|i| parse_export(i, ctx))(input)?;
    
    // エクスポート宣言と関数宣言を分離
    let mut exports = Vec::new();
    let mut declarations = Vec::new();
    
    for (export, decl_opt) in export_results {
        exports.push(export);
        if let Some(decl) = decl_opt {
            declarations.push(decl);
        }
    }
    
    // インポート宣言をパース
    println!("parse_module: インポート宣言のパース開始");
    let (input, imports) = many0(|i| parse_import(i, ctx))(input)?;
    println!("parse_module: インポート宣言のパース終了 imports.len()={}", imports.len());
    for (i, import) in imports.iter().enumerate() {
        match import {
            ImportDecl::Selective { module_path, imports, .. } => {
                println!("parse_module: imports[{}]: Selective {{ module_path: {}, imports.len(): {} }}", i, module_path, imports.len());
                for (j, item) in imports.iter().enumerate() {
                    println!("parse_module:   imports[{}].imports[{}]: {{ name: {}, alias: {:?} }}", i, j, item.name, item.alias);
                }
            },
            ImportDecl::Module { module_path, alias, .. } => {
                println!("parse_module: imports[{}]: Module {{ module_path: {}, alias: {} }}", i, module_path, alias);
            }
        }
    }
    
    // 型宣言をパース
    let (input, type_declarations) = many0(|i| parse_type_declaration(i, ctx))(input)?;
    
    // トレイト宣言をパース
    let (input, trait_declarations) = many0(|i| parse_trait_declaration(i, ctx))(input)?;
    
    // 実装宣言をパース
    let (input, impl_declarations) = many0(|i| parse_impl_declaration(i, ctx))(input)?;
    
    // 文をパース
    let (input, statements) = many0(
        preceded(
            ws_comments(char(';')),
            |i| statement(i, ctx)
        )
    )(input)?;
    
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = ctx.calculate_span(input);
    
    // モジュールスコープを終了
    ctx.exit_scope();
    
    println!("Module path: {}, exports: {}, declarations: {}", path, exports.len(), declarations.len());
    
    Ok((input, Module {
        path,
        exports,
        imports,
        declarations,
        type_declarations,
        trait_declarations,
        impl_declarations,
        statements,
        span,
    }))
}
