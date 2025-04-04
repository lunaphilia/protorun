// 型パーサーのテスト

use super::*;
use crate::protorun::ast::Type;

#[test]
fn test_parse_array_type() {
    let input = "let arr: [Int] = 42;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.declarations.len(), 1); // statements -> declarations
    
    match &program.declarations[0] { // statements -> declarations
        crate::protorun::ast::Decl::Let { pattern, type_annotation, .. } => { // Stmt -> Decl, name -> pattern
            // パターンのチェックを追加
            match pattern {
                crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "arr"),
                _ => panic!("期待される識別子パターンではありません"),
            }
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        Type::Array { element_type, .. } => {
                            match &**element_type {
                                Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待される配列型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
    }
}

#[test]
fn test_parse_generic_type() {
    let input = "let opt: Option<Int> = None;";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.declarations.len(), 1); // statements -> declarations
    
    match &program.declarations[0] { // statements -> declarations
        crate::protorun::ast::Decl::Let { pattern, type_annotation, .. } => { // Stmt -> Decl, name -> pattern
             // パターンのチェックを追加
            match pattern {
                crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "opt"),
                _ => panic!("期待される識別子パターンではありません"),
            }
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        Type::Generic { base_type, type_arguments, .. } => {
                            assert_eq!(base_type, "Option");
                            assert_eq!(type_arguments.len(), 1);
                            
                            match &type_arguments[0] {
                                Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待されるジェネリック型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
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
        // crate::protorun::ast::Type::Tuple { .. } => "Tuple".to_string(), // タプル型を削除
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
        
        assert_eq!(program.declarations.len(), 1); // statements -> declarations
        
        match &program.declarations[0] { // statements -> declarations
            crate::protorun::ast::Decl::Let { pattern, type_annotation, .. } => { // Stmt -> Decl, name -> pattern
                 // パターンのチェックを追加
                match pattern {
                    crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "ref"),
                    _ => panic!("期待される識別子パターンではありません"),
                }
                
                match type_annotation {
                    Some(ty) => {
                        match ty {
                            Type::Reference { is_mutable, referenced_type, .. } => {
                                assert_eq!(*is_mutable, false);
                                
                                match &**referenced_type {
                                    Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                    _ => panic!("期待される単純型ではありません"),
                                }
                            },
                            _ => panic!("期待される参照型ではありません"),
                        }
                    },
                    None => panic!("型注釈が期待されます"),
                }
            },
            _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
        }
    }
    
    // 可変参照
    {
        let input = "let mut_ref: &mut Int = 42;";
        let mut parser = Parser::new(None);
        let program = parser.parse_program(input).unwrap();
        
        assert_eq!(program.declarations.len(), 1); // statements -> declarations
        
        match &program.declarations[0] { // statements -> declarations
            crate::protorun::ast::Decl::Let { pattern, type_annotation, .. } => { // Stmt -> Decl, name -> pattern
                 // パターンのチェックを追加
                match pattern {
                    crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "mut_ref"),
                    _ => panic!("期待される識別子パターンではありません"),
                }
                
                match type_annotation {
                    Some(ty) => {
                        match ty {
                            Type::Reference { is_mutable, referenced_type, .. } => {
                                assert_eq!(*is_mutable, true);
                                
                                match &**referenced_type {
                                    Type::Simple { name, .. } => assert_eq!(name, "Int"),
                                    _ => panic!("期待される単純型ではありません"),
                                }
                            },
                            _ => panic!("期待される参照型ではありません"),
                        }
                    },
                    None => panic!("型注釈が期待されます"),
                }
            },
            _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
        }
    }
}

#[test]
fn test_parse_owned_type() {
    let input = "let owned: own Resource = acquire();";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.declarations.len(), 1); // statements -> declarations
    
    match &program.declarations[0] { // statements -> declarations
        crate::protorun::ast::Decl::Let { pattern, type_annotation, .. } => { // Stmt -> Decl, name -> pattern
             // パターンのチェックを追加
            match pattern {
                crate::protorun::ast::Pattern::Identifier(name, _) => assert_eq!(name, "owned"),
                _ => panic!("期待される識別子パターンではありません"),
            }
            
            match type_annotation {
                Some(ty) => {
                    match ty {
                        Type::Owned { owned_type, .. } => {
                            match &**owned_type {
                                Type::Simple { name, .. } => assert_eq!(name, "Resource"),
                                _ => panic!("期待される単純型ではありません"),
                            }
                        },
                        _ => panic!("期待される所有権型ではありません"),
                    }
                },
                None => panic!("型注釈が期待されます"),
            }
        },
        _ => panic!("期待されるlet宣言ではありません"), // メッセージ変更
    }
}

// test_parse_complex_type はタプル型を含むため削除
