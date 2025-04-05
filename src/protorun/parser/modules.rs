// Protorun言語のモジュールパーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::protorun::ast::{Module, ExportDecl, ImportDecl, ImportItem, Decl};
use super::common::{ParseResult, ws_comments, identifier_string, calculate_span};
// function_declaration を declarations からインポート
use super::declarations::{parse_type_declaration, parse_trait_declaration, parse_impl_declaration, parse_function_declaration};
use super::statements::statement; // statement は statements から

/// インポート種別
enum ImportType {
    Selective(Vec<ImportItem>, String),
    Module(String, String),
}

/// エクスポート宣言のパース
pub fn parse_export<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, (ExportDecl, Option<Decl>)> {
    let (input, _) = ws_comments(tag("export"))(input)?;
    
    // 関数宣言のエクスポート
    let (input, decl) = opt(|i| parse_function_declaration(i, original_input))(input)?; // 関数名を修正
    
    if let Some(decl) = decl {
        let span = calculate_span(original_input, input);
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
                let span = calculate_span(original_input, input);
                (ExportDecl::Group { names, span }, None)
            }
        ),
        // 個別エクスポート（識別子のみ）
        map(
            ws_comments(identifier_string),
            |name| {
                let span = calculate_span(original_input, input);
                (ExportDecl::Single { name, span }, None)
            }
        )
    ))(input)?;
    
    Ok((input, export))
}

/// インポートアイテムのパース
fn parse_import_item<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ImportItem> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // asキーワードの後のスペースを必須にしない
    let (input, alias) = opt(
        preceded(
            ws_comments(tag("as")),
            ws_comments(identifier_string)
        )
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    let result = ImportItem {
        name,
        alias,
        span,
    };
    
    Ok((input, result))
}

/// インポート宣言のパース
pub fn parse_import<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ImportDecl> {
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
                                parse_import_item(i, original_input)
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
                ImportType::Module(module_path, alias)
            }
        )
    ))(input)?;

    let span = calculate_span(original_input, input);
    let import = match import_type {
        ImportType::Selective(imports, module_path) => {
            ImportDecl::Selective {
                module_path,
                imports,
                span,
            }
        },
        ImportType::Module(module_path, alias) => {
            ImportDecl::Module {
                module_path,
                alias,
                span,
            }
        },
    };
    
    Ok((input, import))
}


/// モジュール宣言のパース
pub fn parse_module<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Module> {
    let (input, _) = ws_comments(tag("module"))(input)?;
    let (input, path) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char('{'))(input)?;
    
    // エクスポート宣言をパース
    let (input, exports_and_decls) = many0(|i| parse_export(i, original_input))(input)?;
    
    // エクスポート宣言と関数宣言を分離
    let mut exports = Vec::new();
    let mut declarations = Vec::new();
    
    for (export, decl_opt) in exports_and_decls {
        exports.push(export);
        if let Some(decl) = decl_opt {
            declarations.push(decl);
        }
    }
    
    // インポート宣言をパース
    let (input, imports) = many0(|i| parse_import(i, original_input))(input)?;
    
    // 非エクスポート関数宣言をパース
    let (input, non_export_decls) = many0(|i| parse_function_declaration(i, original_input))(input)?; // 関数名を修正
    
    // 非エクスポート関数宣言を追加
    declarations.extend(non_export_decls);
    
    // 型宣言をパース
    let (input, type_declarations) = many0(|i| parse_type_declaration(i, original_input))(input)?;
    
    // トレイト宣言をパース
    let (input, trait_declarations) = many0(|i| parse_trait_declaration(i, original_input))(input)?;
    
    // 実装宣言をパース
    let (input, impl_declarations) = many0(|i| parse_impl_declaration(i, original_input))(input)?;
    
    // 文をパース
    let (input, statements) = many0(
        preceded(
            ws_comments(char(';')),
            |i| statement(i, original_input)
        )
    )(input)?;
    
    let (input, _) = ws_comments(char('}'))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Module {
        path,
        exports,
        imports,
        declarations,
        type_declarations,
        trait_declarations,
        impl_declarations,
        // statements: statements, // 削除
        expressions: Vec::new(), // モジュール内にトップレベル式はないはずなので空で初期化
        span,
    }))
}
