// Protorun言語の型宣言パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, opt, map}, // map を追加
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple}, // tuple を追加
};

// Decl, Pattern, Expr, Type, Span をインポート
use crate::protorun::ast::{Decl, TypeDecl, EnumVariant, TraitDecl, ImplDecl};
use super::common::{ParseResult, ws_comments, identifier_string, keyword, calculate_span};
use super::types::parse_type;
use super::patterns::pattern as parse_pattern; // パターンパーサーをインポート
use super::expressions::expression; // 式パーサーをインポート
use crate::protorun::ast::Parameter; // Parameter をインポート

/// ジェネリックパラメータのパース
pub fn parse_generic_parameters<'a>(input: &'a str) -> ParseResult<'a, Vec<String>> {
    delimited(
        ws_comments(char('<')),
        separated_list0(
            ws_comments(char(',')),
            ws_comments(identifier_string)
        ),
        cut(ws_comments(char('>')))
    )(input)
}

/// レコード型宣言のパース
pub fn parse_record_type_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TypeDecl> {
    // type Identifier GenericParams? = { field1: Type, field2: Type, ... }
    let (input, _) = keyword("type")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(parse_generic_parameters)(input)?;
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    let (input, _) = ws_comments(tag("="))(input)?;
    
    // レコードフィールドのパース
    let (input, fields) = delimited(
        ws_comments(char('{')),
        separated_list0(
            ws_comments(char(',')),
            pair(
                ws_comments(identifier_string),
                preceded(ws_comments(char(':')), |i| parse_type(i, original_input))
            )
        ),
        cut(ws_comments(char('}')))
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, TypeDecl::Record {
        name,
        type_parameters,
        fields,
        span,
    }))
}

/// enumバリアントのパース
fn parse_enum_variant<'a>(original_input: &'a str) -> impl FnMut(&'a str) -> ParseResult<'a, EnumVariant> + 'a {
    move |input: &'a str| {
        let (input, name) = ws_comments(identifier_string)(input)?;
        
        // バリアントフィールドのパース（オプション）
        let (input, fields) = opt(delimited(
            ws_comments(char('(')),
            separated_list0(
                ws_comments(char(',')),
                |i| parse_type(i, original_input)
            ),
            cut(ws_comments(char(')')))
        ))(input)?;
        
        let fields = fields.unwrap_or_else(Vec::new);
        let span = calculate_span(original_input, input);
        
        Ok((input, EnumVariant {
            name,
            fields,
            span,
        }))
    }
}

/// 代数的データ型（enum）宣言のパース
pub fn parse_enum_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TypeDecl> {
    // enum Identifier GenericParams? { Variant1, Variant2(Type), ... }
    let (input, _) = keyword("enum")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(parse_generic_parameters)(input)?;
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    // enumバリアントのパース
    let (input, variants) = delimited(
        ws_comments(char('{')),
        separated_list0(
            ws_comments(char(',')),
            parse_enum_variant(original_input)
        ),
        cut(ws_comments(char('}')))
    )(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, TypeDecl::Enum {
        name,
        type_parameters,
        variants,
        span,
    }))
}

/// 型エイリアスのパース
pub fn parse_type_alias<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TypeDecl> {
    // type Identifier GenericParams? = Type
    let (input, _) = keyword("type")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(parse_generic_parameters)(input)?;
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    let (input, _) = ws_comments(tag("="))(input)?;
    
    // エイリアスの型をパース
    let (input, aliased_type) = parse_type(input, original_input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, TypeDecl::Alias {
        name,
        type_parameters,
        aliased_type,
        span,
    }))
}

/// トレイト宣言のパース
pub fn parse_trait_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TraitDecl> {
    // trait Identifier GenericParams? : SuperTrait? { fn method(...) ... }
    let (input, _) = keyword("trait")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(parse_generic_parameters)(input)?;
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    // 親トレイトのパース（オプション）
    let (input, super_trait) = opt(preceded(
        ws_comments(char(':')),
        |i| parse_type(i, original_input)
    ))(input)?;
    
    // トレイト本体のパース
    // 注意: 関数宣言のパースは現在のコンテキストでは実装できないため、空のベクターを返す
    // 実際の実装では、関数宣言をパースするための適切な方法を実装する必要がある
    let (input, _) = delimited(
        ws_comments(char('{')),
        many0(ws_comments(identifier_string)), // 仮のパーサー
        cut(ws_comments(char('}')))
    )(input)?;
    
    let methods = Vec::new(); // 空のメソッドリスト
    
    let span = calculate_span(original_input, input);
    
    Ok((input, TraitDecl {
        name,
        type_parameters,
        super_trait,
        methods,
        span,
    }))
}

/// トレイト実装のパース
pub fn parse_impl_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ImplDecl> {
    // impl GenericParams? Type : Trait { fn method(...) ... }
    let (input, _) = keyword("impl")(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(parse_generic_parameters)(input)?;
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    // 実装対象の型をパース
    let (input, target_type) = parse_type(input, original_input)?;
    
    // トレイト型をパース
    let (input, _) = ws_comments(char(':'))(input)?;
    let (input, trait_type) = parse_type(input, original_input)?;
    
    // 実装本体のパース
    // 注意: 関数宣言のパースは現在のコンテキストでは実装できないため、空のベクターを返す
    // 実際の実装では、関数宣言をパースするための適切な方法を実装する必要がある
    let (input, _) = delimited(
        ws_comments(char('{')),
        many0(ws_comments(identifier_string)), // 仮のパーサー
        cut(ws_comments(char('}')))
    )(input)?;
    
    let methods = Vec::new(); // 空のメソッドリスト
    
    let span = calculate_span(original_input, input);
    
    Ok((input, ImplDecl {
        type_parameters,
        target_type,
        trait_type,
        methods,
        span,
    }))
}

/// 型宣言のパース（統合版）
pub fn parse_type_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TypeDecl> {
    alt((
        |i| parse_record_type_declaration(i, original_input),
        |i| parse_enum_declaration(i, original_input),
        |i| parse_type_alias(i, original_input)
    ))(input)
}

/// let宣言のパース
pub fn parse_let_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    map(
        tuple((
            keyword("let"),
            ws_comments(|i| parse_pattern(i, original_input)), // パターンをパース (関数名を修正)
            opt(preceded(
                ws_comments(char(':')),
                |i| parse_type(i, original_input), // 型注釈（オプション）
            )),
            ws_comments(char('=')),
            ws_comments(|i| expression(i, original_input)), // 式をパース
        )),
        move |(_, pattern, type_annotation, _, value)| {
            // Span の計算: 開始は "let" の直前、終了は value の直後
            // calculate_span は input の残りを使うので、開始位置を別途記録するか、
            // あるいは nom-locate などを使うのがより正確だが、ここでは近似的な方法を取る。
            // より正確には、入力全体と残りの入力からスパンを計算するヘルパーを使う。
            // calculate_span がその役割を果たすと仮定する。
            let span = calculate_span(original_input, input); // input はタプルの後の残り
            Decl::Let {
                pattern,
                type_annotation,
                value,
                span,
            }
        }
    )(input)
}

/// var宣言のパース
pub fn parse_var_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    map(
        tuple((
            keyword("var"),
            ws_comments(identifier_string), // var は識別子のみ
            opt(preceded(
                ws_comments(char(':')),
                |i| parse_type(i, original_input), // 型注釈（オプション）
            )),
            ws_comments(char('=')),
            ws_comments(|i| expression(i, original_input)), // 式をパース
        )),
        move |(_, name, type_annotation, _, value)| {
            let span = calculate_span(original_input, input); // input はタプルの後の残り
            Decl::Var {
                name,
                type_annotation,
                value,
                span,
            }
        }
    )(input)
}

/// パラメータをパース
pub fn parameter<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Parameter> {
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, type_annotation) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;

    let span = calculate_span(original_input, input);

    Ok((input, Parameter {
        name,
        type_annotation,
        span,
    }))
}

/// 関数宣言をパース
pub fn parse_function_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    let (input, _) = ws_comments(tag("fn"))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;

    // パラメータをパース
    let (input, parameters) = super::common::delimited_list(
        '(',
        |i| parameter(i, original_input),
        ',',
        ')'
    )(input)?;

    // 戻り値の型（オプション）
    let (input, return_type) = opt(
        preceded(
            ws_comments(char(':')),
            |i| parse_type(i, original_input)
        )
    )(input)?;

    // 関数本体
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = cut(|i| expression(i, original_input))(input)?;
    // let (input, _) = opt(ws_comments(char(';')))(input)?; // セミコロンは不要のはず

    let span = calculate_span(original_input, input);

    Ok((input, Decl::Function {
        name,
        parameters,
        return_type,
        body,
        span,
    }))
}


/// 宣言（Function, Let, Var）のパース
pub fn parse_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    alt((
        |i| parse_function_declaration(i, original_input),
        |i| parse_let_declaration(i, original_input),
        |i| parse_var_declaration(i, original_input),
    ))(input)
}
