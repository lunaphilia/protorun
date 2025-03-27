// Protorun言語の型パーサー

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
};

use crate::protorun::ast::Type;
use super::common::{ParseResult, ws_comments, identifier_string, calculate_span};

/// 単純型をパース
pub fn simple_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    map(
        ws_comments(identifier_string),
        move |name| {
            let span = calculate_span(original_input, input);
            Type::Simple { name, span }
        }
    )(input)
}

/// ジェネリック型引数をパース
pub fn generic_args<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Vec<Type>> {
    delimited(
        ws_comments(char('<')),
        separated_list0(
            ws_comments(char(',')),
            |i| parse_type(i, original_input)
        ),
        cut(ws_comments(char('>')))
    )(input)
}

/// ジェネリック型をパース
pub fn generic_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, base_name) = ws_comments(identifier_string)(input)?;
    let (input, args_opt) = opt(|i| generic_args(i, original_input))(input)?;
    
    match args_opt {
        Some(args) if !args.is_empty() => {
            let span = calculate_span(original_input, input);
            Ok((input, Type::Generic {
                base_type: base_name,
                type_arguments: args,
                span,
            }))
        },
        _ => {
            // ジェネリック引数がない場合は単純型として扱う
            let span = calculate_span(original_input, input);
            Ok((input, Type::Simple { name: base_name, span }))
        }
    }
}

/// 配列型をパース
pub fn array_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, _) = ws_comments(char('['))(input)?;
    let (input, element_type) = parse_type(input, original_input)?;
    let (input, _) = cut(ws_comments(char(']')))(input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Type::Array {
        element_type: Box::new(element_type),
        span,
    }))
}

/// タプル型をパース
pub fn tuple_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, _) = ws_comments(char('('))(input)?;
    let (input, first_type) = parse_type(input, original_input)?;
    
    // カンマがある場合はタプル型、ない場合は括弧で囲まれた型
    let (input, rest) = opt(
        preceded(
            ws_comments(char(',')),
            separated_list0(
                ws_comments(char(',')),
                |i| parse_type(i, original_input)
            )
        )
    )(input)?;
    
    let (input, _) = cut(ws_comments(char(')')))(input)?;
    
    match rest {
        Some(mut types) => {
            // タプル型
            let span = calculate_span(original_input, input);
            
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

/// 効果型をパース
pub fn effect_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    preceded(
        ws_comments(tag("&")),
        |i| parse_type(i, original_input)
    )(input)
}

/// 関数型をパース
pub fn function_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, _) = ws_comments(char('('))(input)?;
    
    // カンマで区切られた型のリストをパース
    let (input, first_type_opt) = opt(|i| parse_type(i, original_input))(input)?;
    
    let (input, params) = match first_type_opt {
        Some(first_type) => {
            let (input, rest_types) = many0(
                preceded(
                    ws_comments(char(',')),
                    |i| parse_type(i, original_input)
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
    let (input, return_type) = parse_type(input, original_input)?;
    
    // オプションの効果型
    let (input, effect) = opt(|i| effect_type(i, original_input))(input)?;
    
    let span = calculate_span(original_input, input);
    
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

/// 参照型をパース
pub fn reference_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, is_mutable) = alt((
        map(preceded(ws_comments(char('&')), ws_comments(tag("mut"))), |_| true),
        map(ws_comments(char('&')), |_| false)
    ))(input)?;
    
    let (input, referenced_type) = parse_type(input, original_input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Type::Reference {
        is_mutable,
        referenced_type: Box::new(referenced_type),
        span,
    }))
}

/// 所有権型をパース
pub fn owned_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    let (input, _) = ws_comments(tag("own"))(input)?;
    let (input, owned_type) = parse_type(input, original_input)?;
    
    let span = calculate_span(original_input, input);
    
    Ok((input, Type::Owned {
        owned_type: Box::new(owned_type),
        span,
    }))
}

/// 型をパース
pub fn parse_type<'a>(input: &'a str, original_input: &'a str) -> ParseResult<'a, Type> {
    alt((
        |i| owned_type(i, original_input),
        |i| reference_type(i, original_input),
        |i| function_type(i, original_input),
        |i| array_type(i, original_input),
        |i| tuple_type(i, original_input),
        |i| generic_type(i, original_input)
    ))(input)
}
