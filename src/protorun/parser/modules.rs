// Protorun言語のモジュールパーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated}, // tuple を削除
};
// VerboseError のインポートは不要になったので削除

use crate::protorun::ast::{Module, ExportDecl, ImportDecl, ImportItem, Decl, Pattern, TypeDecl, TraitDecl, ImplDecl};
use super::common::{ParseResult, ws_comments, identifier_string, calculate_span, keyword};
// parse_declaration をインポートし、他の宣言パーサーも追加
use super::declarations::{parse_declaration, parse_type_declaration, parse_trait_declaration, parse_impl_declaration};

/// インポート種別
enum ImportType {
    Selective(Vec<ImportItem>, String),
    Module(String, String),
}

// parse_export 関数を削除

/// インポートアイテムのパース
fn parse_import_item<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ImportItem> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    
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
    
    let (input, import_type) = alt((
        // 選択的インポート
        map(
            pair( // tuple を pair に変更
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
            ),
            |(imports, module_path)| {
                ImportType::Selective(imports, module_path)
            }
        ),
        // モジュール全体のインポート
        map(
            pair( // tuple を pair に変更
                delimited(
                    ws_comments(char('"')),
                    identifier_string,
                    ws_comments(char('"'))
                ),
                preceded(
                    ws_comments(tag("as")),
                    ws_comments(identifier_string)
                )
            ),
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


/// モジュール内のアイテム
#[derive(Debug)]
enum ModuleItem {
    Import(ImportDecl),
    ExportGroup(ExportDecl),
    Declaration { decl: Decl, is_exported: bool },
    TypeDeclaration { decl: TypeDecl, is_exported: bool },
    TraitDeclaration { decl: TraitDecl, is_exported: bool },
    ImplDeclaration(ImplDecl),
}

/// モジュール内の単一アイテムをパースする関数 (シンプル版)
fn parse_module_item<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ModuleItem> {
    alt((
        // インポート宣言 (export 不可)
        map(ws_comments(|i| parse_import(i, original_input)), ModuleItem::Import),
        // グループエクスポート (export { ... })
        map(
            preceded(
                keyword("export"), // "export" を先にパース
                preceded(
                    ws_comments(char('{')), // '{' をパース
                    cut(terminated(
                        many0(terminated(
                            ws_comments(identifier_string), // 識別子をパース
                            opt(ws_comments(char(','))) // オプションのカンマ
                        )),
                        ws_comments(char('}')) // '}' をパース
                    ))
                )
            ),
            move |names| { // パースされた識別子のリスト (names) を受け取る
                let span = calculate_span(original_input, input); // スパン計算 (より正確な計算が必要かも)
                ModuleItem::ExportGroup(ExportDecl::Group { names, span })
            }
        ),
        // エクスポートされる宣言 (export + 宣言)
        preceded(
            keyword("export"), // "export" を先にパース
            alt(( // 次に来る宣言の種類を alt で試す
                map(ws_comments(|i| parse_declaration(i, original_input)), |d| ModuleItem::Declaration { decl: d, is_exported: true }),
                map(ws_comments(|i| parse_type_declaration(i, original_input)), |d| ModuleItem::TypeDeclaration { decl: d, is_exported: true }),
                map(ws_comments(|i| parse_trait_declaration(i, original_input)), |d| ModuleItem::TraitDeclaration { decl: d, is_exported: true }),
                // impl はエクスポートできないのでここには含めない
            ))
        ),
        // エクスポートされない宣言
        alt((
            map(ws_comments(|i| parse_declaration(i, original_input)), |d| ModuleItem::Declaration { decl: d, is_exported: false }),
            map(ws_comments(|i| parse_type_declaration(i, original_input)), |d| ModuleItem::TypeDeclaration { decl: d, is_exported: false }),
            map(ws_comments(|i| parse_trait_declaration(i, original_input)), |d| ModuleItem::TraitDeclaration { decl: d, is_exported: false }),
            map(ws_comments(|i| parse_impl_declaration(i, original_input)), ModuleItem::ImplDeclaration), // is_exported は false 固定なので不要
        ))
    ))(input)
}


/// モジュール宣言のパース
pub fn parse_module<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Module> {
    let (input, _) = keyword("module")(input)?;
    let (input, path) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char('{'))(input)?;

    let mut current_input = input;
    let mut items = Vec::new();

    // モジュール内のアイテムをループでパース
    loop {
        // 空白とコメントをスキップ
        let (next_input, _) = ws_comments::<_, ()>(|i| Ok((i, ())))(current_input)?;

        // ループ終了条件: '}' が見つかったら終了
        if next_input.starts_with('}') {
            current_input = next_input;
            break;
        }
        // ループが終了しない場合のエラーハンドリング（無限ループ防止）
        if next_input.is_empty() {
             use nom::error::VerboseErrorKind; // エラー種類をインポート
             return Err(nom::Err::Error(nom::error::VerboseError{ errors: vec![(next_input, VerboseErrorKind::Context("Unexpected EOF in module body"))]}));
        }


        // 次のアイテムをパース
        match parse_module_item(next_input, original_input) {
            Ok((next_input_after_item, item)) => {
                items.push(item);
                current_input = next_input_after_item;
            }
            Err(e) => {
                 return Err(e); // エラーを返す
            }
        }
    }


    let (input, _) = cut(ws_comments(char('}')))(current_input)?;

    // パース結果を Module 構造体に格納
    let mut exports = Vec::new();
    let mut imports = Vec::new();
    let mut declarations = Vec::new();
    let mut type_declarations = Vec::new();
    let mut trait_declarations = Vec::new();
    let mut impl_declarations = Vec::new();

    for item in items {
        match item {
            ModuleItem::Import(i) => imports.push(i),
            ModuleItem::ExportGroup(e) => exports.push(e),
            ModuleItem::Declaration { decl, is_exported } => {
                if is_exported {
                    let name_span_opt: Option<(String, crate::protorun::ast::Span)> = match &decl {
                        Decl::Let { pattern, span, .. } => match pattern {
                            Pattern::Identifier(name, _) => Some((name.clone(), span.clone())),
                            _ => None,
                        },
                        Decl::Var { name, span, .. } => Some((name.clone(), span.clone())),
                        Decl::HandlerDecl(_) => None, // HandlerDecl は単一の名前を持たないため None を返す
                    };

                    if let Some((name, span)) = name_span_opt {
                        exports.push(ExportDecl::Single { name, span });
                    // HandlerDecl の場合は is_exported が true でも name_span_opt は None になるので警告しない
                    } else if is_exported && !matches!(decl, Decl::HandlerDecl(_)) {
                         // is_exported が true なのに name_span_opt が None の場合
                         // (例: export let (a, b) = ...)
                         eprintln!("Warning: export keyword used with non-exportable declaration pattern.");
                    }
                }
                declarations.push(decl);
            },
            ModuleItem::TypeDeclaration { decl, is_exported } => {
                if is_exported {
                    let (name, span) = match &decl {
                        TypeDecl::Record { name, span, .. } => (name.clone(), span.clone()),
                        TypeDecl::Enum { name, span, .. } => (name.clone(), span.clone()),
                        TypeDecl::Alias { name, span, .. } => (name.clone(), span.clone()),
                    };
                    exports.push(ExportDecl::Single { name, span });
                }
                type_declarations.push(decl);
            },
             ModuleItem::TraitDeclaration { decl, is_exported } => {
                if is_exported {
                    let name = decl.name.clone();
                    let span = decl.span.clone();
                    exports.push(ExportDecl::Single { name, span });
                }
                trait_declarations.push(decl);
            },
            ModuleItem::ImplDeclaration(id) => impl_declarations.push(id),
        }
    }
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Module {
        path,
        exports,
        imports,
        declarations,
        type_declarations,
        trait_declarations,
        impl_declarations,
        expressions: Vec::new(),
        span,
    }))
}
