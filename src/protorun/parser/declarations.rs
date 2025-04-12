// Protorun言語の型宣言パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, opt, map}, // map を追加
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple, terminated}, // terminated を追加
};

// AST ノードをインポート
use crate::protorun::ast::{
    Decl, TypeDecl, EnumVariant, TraitDecl, ImplDecl, HandlerDecl, HandlerMember, FieldDecl,
    LetHandlerFunction, HandlerFunctionBody, ResumeFunctionExpr, NoResumeFunctionExpr,
    GenericParam, // ResumeType を削除
};
use super::common::{ParseResult, ws_comments, identifier_string, keyword, calculate_span, parameter, delimited_list, consume_ws_comments}; // parameter, delimited_list, consume_ws_comments をインポート
use super::types::parse_type; // parse_generic_params のインポートを削除
use super::patterns::pattern as parse_pattern; // パターンパーサーをインポート
use super::expressions::{expression, function_expr}; // expression, function_expr をインポート

/// ジェネリックパラメータのパース (AST ノードを返すように変更)
pub fn parse_generic_params<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Vec<GenericParam>> {
    map(
        delimited(
            ws_comments(char('<')),
            separated_list0(
                ws_comments(char(',')),
                // 識別子とオプションの型制約をパース
                pair(
                    ws_comments(identifier_string),
                    opt(preceded(
                        ws_comments(char(':')),
                        |i| parse_type(i, original_input) // 型制約をパース
                    ))
                )
            ),
            cut(ws_comments(char('>')))
        ),
        move |params| {
            params.into_iter().map(|(name, constraints)| {
                let span = calculate_span(original_input, input); // スパン計算は要改善
                GenericParam { name, constraints, span } // constraints フィールドを初期化
            }).collect()
        }
    )(input)
}

/// レコード型宣言のパース
pub fn parse_record_type_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, TypeDecl> {
    // type Identifier GenericParams? = { field1: Type, field2: Type, ... }
    let (input, _) = keyword("type")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    
    // ジェネリックパラメータのパース（オプション）
    let (input, type_parameters) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back for clarity, will fix import later if needed
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
    let (input, type_parameters) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
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
    let (input, type_parameters) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
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
    let (input, type_parameters) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    // 親トレイトのパース（オプション）
    let (input, super_trait) = opt(preceded(
        ws_comments(char(':')),
        |i| parse_type(i, original_input)
    ))(input)?;
    
    // トレイト本体のパース: { Decl* }
    let (input, methods) = delimited(
        ws_comments(char('{')),
        // メソッド定義 (Decl) を 0 個以上パース
        many0(
            // 各宣言の後には空白/コメントが続くことを想定
            terminated(
                |i| parse_declaration(i, original_input), // Decl をパース
                ws_comments(opt(char(';'))) // メソッド定義後のセミコロンはオプション
            )
        ),
        cut(ws_comments(char('}')))
    )(input)?;
    
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
    let (input, type_parameters) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
    let type_parameters = type_parameters.unwrap_or_else(Vec::new);
    
    // 実装対象の型をパース
    let (input, target_type) = parse_type(input, original_input)?;
    
    // トレイト型をパース
    let (input, _) = ws_comments(char(':'))(input)?;
    let (input, trait_type) = parse_type(input, original_input)?;
    
    // 実装本体のパース: { Decl* }
    let (input, methods) = delimited(
        ws_comments(char('{')),
        // メソッド定義 (Decl) を 0 個以上パース
        many0(
            // 各宣言の後には空白/コメントが続くことを想定
            terminated(
                |i| parse_declaration(i, original_input), // Decl をパース
                ws_comments(opt(char(';'))) // メソッド定義後のセミコロンはオプション
            )
        ),
        cut(ws_comments(char('}')))
    )(input)?;
    
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
            // expression を呼び出す前にデバッグプリント追加
            |i: &'a str| { println!("--- parse_let_declaration: before expression ---"); dbg!(i); expression(i, original_input) },
        )),
        move |(_, pattern, type_annotation, _, value)| {
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

// parameter 関数は common.rs に移動

// parse_function_declaration 関数は削除

/// 宣言（Let, Var）のパース
pub fn parse_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    alt((
        // parse_function_declaration の呼び出しを削除
        |i| parse_let_declaration(i, original_input),
        |i| parse_var_declaration(i, original_input),
        |i| parse_handler_declaration(i, original_input), // ハンドラ宣言を追加
    ))(input)
}

/// ハンドラ内のフィールド宣言をパース: (let | var) Identifier : Type
fn parse_field_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, FieldDecl> {
    let (input, is_mutable) = alt((
        map(keyword("var"), |_| true),
        map(keyword("let"), |_| false),
    ))(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, _) = ws_comments(char(':'))(input)?;
    let (input, type_annotation) = parse_type(input, original_input)?;
    let span = calculate_span(original_input, input);
    Ok((input, FieldDecl { is_mutable, name, type_annotation, span }))
}

// /// Resume 型のパース (文法変更により削除)
// fn parse_resume_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ResumeType> {
//     map(
//         tuple((
//             delimited_list('(', |i| parse_type(i, original_input), ',', ')'),
//             preceded(ws_comments(tag("->")), |i| parse_type(i, original_input)),
//         )),
//         move |(parameters, return_type)| {
//             let span = calculate_span(original_input, input); // スパン計算は要改善
//             ResumeType { parameters, return_type: Box::new(return_type), span }
//         }
//     )(input)
// }


/// resume 付きハンドラ関数本体のパース (文法修正: "fn" ParamList "resume" (":" ReturnType)? "=>" Expression)
fn parse_resume_function_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, ResumeFunctionExpr> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, _) = keyword("fn")(input)?;
    let (input, parameters) = delimited_list('(', |i| parameter(i, original_input), ',', ')')(input)?;
    // let (input, _) = ws_comments(char(','))(input)?; // カンマ削除
    let (input, _) = keyword("resume")(input)?;
    // let (input, _) = ws_comments(char(':'))(input)?; // コロン削除
    // let (input, resume_type) = parse_resume_type(input, original_input)?; // ResumeType パース削除
    let (input, return_type) = opt(preceded(ws_comments(char(':')), |i| parse_type(i, original_input)))(input)?; // オプションの戻り値型
    let (input, _) = ws_comments(tag("=>"))(input)?;
    let (input, body) = expression(input, original_input)?;
    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);
    Ok((input, ResumeFunctionExpr { parameters, /*resume_type,*/ return_type, body: Box::new(body), span })) // resume_type 削除
}

/// noresume 付きハンドラ関数本体のパース (文法修正: "fn" ParamList "noresume" (":" ReturnType)? "=>" Expression)
fn parse_no_resume_function_expr<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, NoResumeFunctionExpr> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, _) = keyword("fn")(input)?;
    let (input, parameters) = delimited_list('(', |i| parameter(i, original_input), ',', ')')(input)?;
    // let (input, _) = ws_comments(char(':'))(input)?; // コロン削除
    let (input, _) = keyword("noresume")(input)?;
    let (input, return_type) = opt(preceded(ws_comments(char(':')), |i| parse_type(i, original_input)))(input)?; // オプションの戻り値型に変更
    let (input, _) = ws_comments(tag("=>"))(input)?;
    let (input, body) = expression(input, original_input)?;
    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);
    Ok((input, NoResumeFunctionExpr { parameters, return_type, body: Box::new(body), span })) // return_type は Option<Type>
}

/// ハンドラ関数本体のパース
fn parse_handler_function_body<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, HandlerFunctionBody> {
    alt((
        map(|i| parse_resume_function_expr(i, original_input), HandlerFunctionBody::ResumeFunction),
        map(|i| parse_no_resume_function_expr(i, original_input), HandlerFunctionBody::NoResumeFunction),
        map(|i| function_expr(i, original_input), HandlerFunctionBody::Function), // 通常の FunctionExpr
    ))(input)
}


/// ハンドラ内の関数束縛のパース: let Identifier GenericParams? = HandlerFunctionBody
fn parse_let_handler_function<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, LetHandlerFunction> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, _) = keyword("let")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, generic_params) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
    let (input, _) = ws_comments(char('='))(input)?;
    let (input, body) = parse_handler_function_body(input, original_input)?;
    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]);
    Ok((input, LetHandlerFunction { name, generic_params, body, span }))
}

/// ハンドラメンバーのパース (FieldDecl | LetHandlerFunction)
fn parse_handler_member<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, HandlerMember> {
    alt((
        map(|i| parse_field_declaration(i, original_input), HandlerMember::Field),
        map(|i| parse_let_handler_function(i, original_input), HandlerMember::Function),
    ))(input)
}

/// ハンドラ宣言のパース: handler Identifier GenericParams? : EffectType { HandlerMember* }
pub fn parse_handler_declaration<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Decl> {
    let start_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let (input, _) = keyword("handler")(input)?;
    let (input, name) = ws_comments(identifier_string)(input)?;
    let (input, generic_params) = opt(|i| parse_generic_params(i, original_input))(input)?; // Renamed back
    let (input, _) = ws_comments(char(':'))(input)?;
    let (input, effect_type) = parse_type(input, original_input)?;
    let (input, _) = ws_comments(char('{'))(input)?; // Consume opening brace

    let mut current_input = input;
    let mut members = Vec::new();

    // モジュール内のアイテムをループでパース
    loop {
        // Consume leading whitespace/comments for the next potential member
        let (next_input, _) = consume_ws_comments(current_input)?;

        // Check for closing brace
        if next_input.starts_with('}') {
            current_input = next_input;
            break;
        }
        // EOF check
        if next_input.is_empty() {
             use nom::error::VerboseErrorKind;
             return Err(nom::Err::Error(nom::error::VerboseError{ errors: vec![(next_input, VerboseErrorKind::Context("Unexpected EOF in handler body"))]}));
        }

        // Parse the member itself
        match parse_handler_member(next_input, original_input) {
            Ok((input_after_member, member)) => {
                members.push(member);
                current_input = input_after_member; // Update input *after* parsing member
            }
            Err(e) => return Err(e),
        }
    }

    let (input, _) = cut(ws_comments(char('}')))(current_input)?; // Consume closing brace

    let end_pos = input.as_ptr() as usize - original_input.as_ptr() as usize;
    let span = calculate_span(original_input, &original_input[start_pos..end_pos]); // Use final input for span end

    Ok((input, Decl::HandlerDecl(HandlerDecl {
        name,
        generic_params,
        effect_type,
        members,
        span,
    })))
}
