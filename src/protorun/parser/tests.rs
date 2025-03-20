// パーサーモジュールのテスト

use super::*;
use crate::protorun::ast::{BinaryOperator, Expr, Stmt, UnaryOperator};
use crate::protorun::error::ErrorKind;

#[test]
fn test_parse_expr_statement() {
    let input = "42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.declarations.len(), 0);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Expr { expr, .. } => {
            match expr {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される式文ではありません"),
    }
}

#[test]
fn test_parse_block_expr() {
    let input = "{ let x = 10; x }";
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::Identifier(name, _) => assert_eq!(name, "x"),
        _ => panic!("ブロック式の最後の式がIdentifierではありません"),
    }
}

#[test]
fn test_parse_nested_block_expr() {
    let input = "{ let x = 10; { let y = 20; x + y } }";
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
        _ => panic!("ネストされたブロック式の最後の式がBinaryOpではありません"),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "add(10, 20)";
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::FunctionCall { function, arguments, .. } => {
            match &*function {
                Expr::Identifier(name, _) => assert_eq!(name, "add"),
                _ => panic!("期待される関数名識別子ではありません"),
            }
            
            assert_eq!(arguments.len(), 2);
            
            match &arguments[0] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 10),
                _ => panic!("期待される整数リテラルではありません"),
            }
            
            match &arguments[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 20),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される関数呼び出しではありません"),
    }
}

#[test]
fn test_parse_nested_function_call() {
    let input = "add(multiply(10, 2), 20)";
    let mut parser = Parser::new(None);
    
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::FunctionCall { function, arguments, .. } => {
            match &*function {
                Expr::Identifier(name, _) => assert_eq!(name, "add"),
                _ => panic!("期待される関数名識別子ではありません"),
            }
            
            assert_eq!(arguments.len(), 2);
            
            match &arguments[0] {
                Expr::FunctionCall { function, arguments, .. } => {
                    match &**function {
                        Expr::Identifier(name, _) => assert_eq!(name, "multiply"),
                        _ => panic!("期待される関数名識別子ではありません"),
                    }
                    
                    assert_eq!(arguments.len(), 2);
                    
                    match &arguments[0] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 10),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &arguments[1] {
                        Expr::IntLiteral(value, _) => assert_eq!(*value, 2),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                },
                _ => panic!("期待される関数呼び出しではありません"),
            }
            
            match &arguments[1] {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 20),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される関数呼び出しではありません"),
    }
}

#[test]
fn test_parse_arithmetic_expressions() {
    // 加算
    {
        let input = "1 + 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Add),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 減算
    {
        let input = "5 - 3";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Sub),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 乗算
    {
        let input = "4 * 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mul),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 除算
    {
        let input = "10 / 2";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Div),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 剰余
    {
        let input = "10 % 3";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Mod),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_unary_expressions() {
    // 負の数
    {
        let input = "-42";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::UnaryOp { operator, expr, .. } => {
                assert_eq!(operator, UnaryOperator::Neg);
                
                match *expr {
                    Expr::IntLiteral(value, _) => assert_eq!(value, 42),
                    _ => panic!("期待される整数リテラルではありません"),
                }
            },
            _ => panic!("期待される単項演算ではありません"),
        }
    }
    
    // 論理否定
    {
        let input = "!true";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::UnaryOp { operator, expr, .. } => {
                assert_eq!(operator, UnaryOperator::Not);
                
                match *expr {
                    Expr::BoolLiteral(value, _) => assert_eq!(value, true),
                    _ => panic!("期待される真偽値リテラルではありません"),
                }
            },
            _ => panic!("期待される単項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_comparison_expressions() {
    // 等価
    {
        let input = "x == y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Eq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 非等価
    {
        let input = "x != y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Neq),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より小さい
    {
        let input = "x < y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // より大きい
    {
        let input = "x > y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gt),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以下
    {
        let input = "x <= y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Lte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
    
    // 以上
    {
        let input = "x >= y";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, BinaryOperator::Gte),
            _ => panic!("期待される二項演算ではありません"),
        }
    }
}

#[test]
fn test_parse_parenthesized_expr() {
    let input = "(1 + 2) * 3";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::BinaryOp { operator, left, right, .. } => {
            assert_eq!(operator, BinaryOperator::Mul);
            
            match &*left {
                Expr::ParenExpr(inner, _) => {
                    match &**inner {
                        Expr::BinaryOp { operator, .. } => assert_eq!(*operator, BinaryOperator::Add),
                        _ => panic!("カッコ内の式が期待される二項演算ではありません"),
                    }
                },
                _ => panic!("期待されるカッコ式ではありません"),
            }
            
            match &*right {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 3),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される二項演算ではありません"),
    }
}

#[test]
fn test_parse_let_with_type_annotation() {
    let input = "let x: Int = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, value, .. } => {
            assert_eq!(name, "x");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                        _ => panic!("期待される単純型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
            
            match value {
                Expr::IntLiteral(value, _) => assert_eq!(*value, 42),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_complex_expression() {
    let input = "1 + 2 * 3 + 4";
    let mut parser = Parser::new(None);
    let expr = parser.parse_expression(input).unwrap();
    
    match expr {
        Expr::BinaryOp { operator, left, right, .. } => {
            assert_eq!(operator, BinaryOperator::Add);
            
            match &*left {
                Expr::BinaryOp { operator, left, right, .. } => {
                    assert_eq!(*operator, BinaryOperator::Add);
                    
                    match &**left {
                        Expr::IntLiteral(value, _) => assert_eq!(value, &1),
                        _ => panic!("期待される整数リテラルではありません"),
                    }
                    
                    match &**right {
                        Expr::BinaryOp { operator, left, right, .. } => {
                            assert_eq!(*operator, BinaryOperator::Mul);
                            
                            match &**left {
                                Expr::IntLiteral(value, _) => assert_eq!(value, &2),
                                _ => panic!("期待される整数リテラルではありません"),
                            }
                            
                            match &**right {
                                Expr::IntLiteral(value, _) => assert_eq!(value, &3),
                                _ => panic!("期待される整数リテラルではありません"),
                            }
                        },
                        _ => panic!("期待される二項演算ではありません"),
                    }
                },
                _ => panic!("期待される二項演算ではありません"),
            }
            
            match &*right {
                Expr::IntLiteral(value, _) => assert_eq!(value, &4),
                _ => panic!("期待される整数リテラルではありません"),
            }
        },
        _ => panic!("期待される二項演算ではありません"),
    }
}

#[test]
fn test_parse_error_unexpected_token() {
    let input = "let x = ;";
    let mut parser = Parser::new(None);
    let result = parser.parse_program(input);
    
    assert!(result.is_err());
    
    match result {
        Ok(_) => panic!("エラーが期待されます"),
        Err(err) => {
            match err.kind {
                ErrorKind::Syntax(_) => (), // 期待される構文エラー
                _ => panic!("期待される構文エラーではありません"),
            }
        }
    }
}

#[test]
fn test_parse_string_literal() {
    let input = r#"let message = "Hello, world!";"#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, value, .. } => {
            assert_eq!(name, "message");
            
            match value {
                Expr::StringLiteral(value, _) => assert_eq!(value, "Hello, world!"),
                _ => panic!("期待される文字列リテラルではありません"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_bool_literal() {
    // true
    {
        let input = "true";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, true),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
    
    // false
    {
        let input = "false";
        let mut parser = Parser::new(None);
        let expr = parser.parse_expression(input).unwrap();
        
        match expr {
            Expr::BoolLiteral(value, _) => assert_eq!(value, false),
            _ => panic!("期待される真偽値リテラルではありません"),
        }
    }
}

// 複合型のテスト

#[test]
fn test_parse_array_type() {
    let input = "let arr: [Int] = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, .. } => {
            assert_eq!(name, "arr");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Array { element_type, .. } => {
                            match &**element_type {
                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待される配列型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_tuple_type() {
    let input = "let pair: (Int, String) = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, .. } => {
            assert_eq!(name, "pair");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Tuple { element_types, .. } => {
                            assert_eq!(element_types.len(), 2);
                            
                            match &element_types[0] {
                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                            
                            match &element_types[1] {
                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "String"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待されるタプル型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_generic_type() {
    let input = "let opt: Option<Int> = None;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, .. } => {
            assert_eq!(name, "opt");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Generic { base_type, type_arguments, .. } => {
                            assert_eq!(base_type, "Option");
                            assert_eq!(type_arguments.len(), 1);
                            
                            match &type_arguments[0] {
                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待されるジェネリック型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_function_type() {
    let input = "let add: (Int, Int) -> Int = fn(a, b) = a + b;";
    let mut parser = Parser::new(None);
    
    // 入力文字列を出力
    println!("入力文字列: {}", input);
    
    // 型注釈部分だけを解析
    let type_str = "(Int, Int) -> Int";
    println!("型注釈部分: {}", type_str);
    
    let result = parser.parse_type(type_str);
    
    match result {
        Ok(ty) => {
            println!("解析結果の型: {}", type_name_of(&ty));
            
            match ty {
                crate::protorun::ast::Type::Function { parameters, return_type, .. } => {
                    // パラメータ数を出力
                    println!("パラメータ数: {}", parameters.len());
                    
                    // パラメータの内容を出力
                    for (i, param) in parameters.iter().enumerate() {
                        println!("パラメータ[{}]: {}", i, type_name_of(param));
                    }
                    
                    // 戻り値型を出力
                    println!("戻り値型: {}", type_name_of(return_type.as_ref()));
                },
                _ => println!("関数型ではありません: {}", type_name_of(&ty)),
            }
        },
        Err(e) => println!("解析エラー: {:?}", e),
    }
}

#[test]
fn test_parse_function_with_effect_type() {
    let input = "let readFile: (String) -> String & IO = fn(path) = read(path);";
    let mut parser = Parser::new(None);
    
    // 入力文字列を出力
    println!("入力文字列: {}", input);
    
    // 型注釈部分だけを解析
    let type_str = "(String) -> String & IO";
    println!("型注釈部分: {}", type_str);
    
    let result = parser.parse_type(type_str);
    
    match result {
        Ok(ty) => {
            println!("解析結果の型: {}", type_name_of(&ty));
            
            match ty {
                crate::protorun::ast::Type::WithEffect { base_type, effect_type, .. } => {
                    println!("効果型: {}", type_name_of(effect_type.as_ref()));
                    
                    match base_type.as_ref() {
                        crate::protorun::ast::Type::Function { parameters, return_type, .. } => {
                            // パラメータ数を出力
                            println!("パラメータ数: {}", parameters.len());
                            
                            // パラメータの内容を出力
                            for (i, param) in parameters.iter().enumerate() {
                                println!("パラメータ[{}]: {}", i, type_name_of(param));
                            }
                            
                            // 戻り値型を出力
                            println!("戻り値型: {}", type_name_of(return_type.as_ref()));
                        },
                        _ => println!("関数型ではありません: {}", type_name_of(base_type.as_ref())),
                    }
                },
                _ => println!("効果付き型ではありません: {}", type_name_of(&ty)),
            }
        },
        Err(e) => println!("解析エラー: {:?}", e),
    }
}

// 型の名前を取得する関数
fn type_name_of(ty: &crate::protorun::ast::Type) -> String {
    match ty {
        crate::protorun::ast::Type::Simple { name, .. } => format!("Simple({})", name),
        crate::protorun::ast::Type::Array { .. } => "Array".to_string(),
        crate::protorun::ast::Type::Tuple { .. } => "Tuple".to_string(),
        crate::protorun::ast::Type::Function { .. } => "Function".to_string(),
        crate::protorun::ast::Type::Generic { base_type, .. } => format!("Generic({})", base_type),
        crate::protorun::ast::Type::Reference { is_mutable, .. } => {
            if *is_mutable {
                "Reference(mut)".to_string()
            } else {
                "Reference".to_string()
            }
        },
        crate::protorun::ast::Type::Owned { .. } => "Owned".to_string(),
        crate::protorun::ast::Type::WithEffect { .. } => "WithEffect".to_string(),
    }
}

#[test]
fn test_parse_reference_type() {
    // 不変参照
    {
        let input = "let ref: &Int = 42;";
        let mut parser = Parser::new(None);
        let program = parser.parse_program(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Let { name, type_annotation, .. } => {
                assert_eq!(name, "ref");
                
                match type_annotation {
                    Some(ty) => {
                        match ty {
                            crate::protorun::ast::Type::Reference { is_mutable, referenced_type, .. } => {
                                assert_eq!(*is_mutable, false);
                                
                                match &**referenced_type {
                                    crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                    _ => panic!("期待される単純型ではありません"),
                                }
                            },
                            _ => panic!("期待される参照型ではありません"),
                        }
                    },
                    None => panic!("型注釈が期待されます"),
                }
            },
            _ => panic!("期待されるlet文ではありません"),
        }
    }
    
    // 可変参照
    {
        let input = "let mut_ref: &mut Int = 42;";
        let mut parser = Parser::new(None);
        let program = parser.parse_program(input).unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Let { name, type_annotation, .. } => {
                assert_eq!(name, "mut_ref");
                
                match type_annotation {
                    Some(ty) => {
                        match ty {
                            crate::protorun::ast::Type::Reference { is_mutable, referenced_type, .. } => {
                                assert_eq!(*is_mutable, true);
                                
                                match &**referenced_type {
                                    crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                    _ => panic!("期待される単純型ではありません"),
                                }
                            },
                            _ => panic!("期待される参照型ではありません"),
                        }
                    },
                    None => panic!("型注釈が期待されます"),
                }
            },
            _ => panic!("期待されるlet文ではありません"),
        }
    }
}

#[test]
fn test_parse_owned_type() {
    let input = "let owned: own Resource = acquire();";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, .. } => {
            assert_eq!(name, "owned");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Owned { owned_type, .. } => {
                            match &**owned_type {
                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Resource"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待される所有権型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}

#[test]
fn test_parse_complex_type() {
    let input = "let complex: Option<(Int, &mut String)> = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::Let { name, type_annotation, .. } => {
            assert_eq!(name, "complex");
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        crate::protorun::ast::Type::Generic { base_type, type_arguments, .. } => {
                            assert_eq!(base_type, "Option");
                            assert_eq!(type_arguments.len(), 1);
                            
                            match &type_arguments[0] {
                                crate::protorun::ast::Type::Tuple { element_types, .. } => {
                                    assert_eq!(element_types.len(), 2);
                                    
                                    match &element_types[0] {
                                        crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                        _ => panic!("期待される単純型ではありません"),
                                    }
                                    
                                    match &element_types[1] {
                                        crate::protorun::ast::Type::Reference { is_mutable, referenced_type, .. } => {
                                            assert_eq!(*is_mutable, true);
                                            
                                            match &**referenced_type {
                                                crate::protorun::ast::Type::Simple { name, .. } => assert_eq!(name, "String"),
                                                _ => panic!("期待される単純型ではありません"),
                                            }
                                        },
                                        _ => panic!("期待される参照型ではありません"),
                                    }
                                },
                                _ => panic!("期待されるタプル型ではありません"),
                            }
                        },
                        _ => panic!("期待されるジェネリック型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet文ではありません"),
    }
}
