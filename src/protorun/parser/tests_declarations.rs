// 宣言パーサーのテスト

use super::*;
// 必要な AST ノードをインポート
use crate::protorun::ast::{
    Decl, TypeDecl, Type, HandlerMember, HandlerFunctionBody, Expr,
};

#[test]
fn test_parse_record_type_declaration() {
    let input = "type Person = { name: String, age: Int }";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.type_declarations.len(), 1);
    
    match &program.type_declarations[0] {
        TypeDecl::Record { name, fields, .. } => {
            assert_eq!(name, "Person");
            assert_eq!(fields.len(), 2);
            
            assert_eq!(fields[0].0, "name");
            match &fields[0].1 {
                Type::Simple { name, .. } => assert_eq!(name, "String"),
                _ => panic!("期待される単純型ではありません"),
            }
            
            assert_eq!(fields[1].0, "age");
            match &fields[1].1 {
                Type::Simple { name, .. } => assert_eq!(name, "Int"),
                _ => panic!("期待される単純型ではありません"),
            }
        },
        _ => panic!("期待されるレコード型宣言ではありません"),
    }
}

#[test]
fn test_parse_enum_declaration() {
    let input = "enum Option<T> { Some(T), None }";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.type_declarations.len(), 1);
    
    match &program.type_declarations[0] {
        TypeDecl::Enum { name, type_parameters, variants, .. } => {
            assert_eq!(name, "Option");
            assert_eq!(type_parameters.len(), 1);
            assert_eq!(type_parameters[0].name, "T"); // Compare name field of GenericParam
            assert_eq!(variants.len(), 2);
            
            assert_eq!(variants[0].name, "Some");
            assert_eq!(variants[0].fields.len(), 1);
            
            assert_eq!(variants[1].name, "None");
            assert_eq!(variants[1].fields.len(), 0);
        },
        _ => panic!("期待されるenum型宣言ではありません"),
    }
}

#[test]
fn test_parse_type_alias() {
    let input = "type StringMap<T> = Map<String, T>";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.type_declarations.len(), 1);
    
    match &program.type_declarations[0] {
        TypeDecl::Alias { name, type_parameters, aliased_type, .. } => {
            assert_eq!(name, "StringMap");
            assert_eq!(type_parameters.len(), 1);
            assert_eq!(type_parameters[0].name, "T"); // Compare name field of GenericParam
            
            match aliased_type {
                Type::Generic { base_type, type_arguments, .. } => {
                    assert_eq!(base_type, "Map");
                    assert_eq!(type_arguments.len(), 2);
                    
                    match &type_arguments[0] {
                        Type::Simple { name, .. } => assert_eq!(name, "String"),
                        _ => panic!("期待される単純型ではありません"),
                    }
                    
                    match &type_arguments[1] {
                        Type::Simple { name, .. } => assert_eq!(name, "T"),
                        _ => panic!("期待される単純型ではありません"),
                    }
                },
                _ => panic!("期待されるジェネリック型ではありません"),
            }
        },
        _ => panic!("期待される型エイリアスではありません"),
    }
}

#[test]
fn test_parse_trait_declaration() {
    let input = "trait Show { }";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.trait_declarations.len(), 1);
    
    let trait_decl = &program.trait_declarations[0];
    assert_eq!(trait_decl.name, "Show");
    assert_eq!(trait_decl.type_parameters.len(), 0);
    assert!(trait_decl.super_trait.is_none());
    assert_eq!(trait_decl.methods.len(), 0);
}

#[test]
fn test_parse_trait_with_super_trait() {
    let input = "trait Ord: Eq { }";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.trait_declarations.len(), 1);
    
    let trait_decl = &program.trait_declarations[0];
    assert_eq!(trait_decl.name, "Ord");
    assert!(trait_decl.super_trait.is_some());
    
    match &trait_decl.super_trait {
        Some(Type::Simple { name, .. }) => assert_eq!(name, "Eq"),
        _ => panic!("期待される単純型ではありません"),
    }
}

#[test]
fn test_parse_impl_declaration() {
    let input = "impl Int: Show { }";
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.impl_declarations.len(), 1);
    
    let impl_decl = &program.impl_declarations[0];
    
    match &impl_decl.target_type {
        Type::Simple { name, .. } => assert_eq!(name, "Int"),
        _ => panic!("期待される単純型ではありません"),
    }
    
    match &impl_decl.trait_type {
        Type::Simple { name, .. } => assert_eq!(name, "Show"),
        _ => panic!("期待される単純型ではありません"),
    }
    
    assert_eq!(impl_decl.methods.len(), 0);
}

#[test]
fn test_parse_handler_declaration_simple() {
    let input = r#"
        handler ConsoleHandler: Console {
            let log = fn (msg: String) => println(msg)
            let read = fn () => readLine()
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();

    assert_eq!(program.declarations.len(), 1);
    match &program.declarations[0] {
        Decl::HandlerDecl(decl) => {
            assert_eq!(decl.name, "ConsoleHandler");
            assert!(decl.generic_params.is_none());
            match &decl.effect_type {
                Type::Simple { name, .. } => assert_eq!(name, "Console"),
                _ => panic!("Expected simple effect type"),
            }
            assert_eq!(decl.members.len(), 2);
            match &decl.members[0] {
                HandlerMember::Function(f) => {
                    assert_eq!(f.name, "log");
                    assert!(f.generic_params.is_none());
                    match &f.body {
                        HandlerFunctionBody::Function(Expr::FunctionExpr { parameters, .. }) => {
                             assert!(parameters.is_some());
                             assert_eq!(parameters.as_ref().unwrap().len(), 1);
                             assert_eq!(parameters.as_ref().unwrap()[0].name, "msg");
                        }
                        _ => panic!("Expected Function body"),
                    }
                }
                _ => panic!("Expected Function member"),
            }
             match &decl.members[1] {
                 HandlerMember::Function(f) => {
                    assert_eq!(f.name, "read");
                     match &f.body {
                        HandlerFunctionBody::Function(Expr::FunctionExpr { parameters, .. }) => {
                             // パラメータがない場合は Some(vec![]) になるはず
                             assert!(parameters.is_some());
                             assert!(parameters.as_ref().unwrap().is_empty());
                        }
                        _ => panic!("Expected Function body"),
                    }
                }
                _ => panic!("Expected Function member"),
            }
        }
        _ => panic!("Expected HandlerDecl"),
    }
}

#[test]
fn test_parse_handler_declaration_with_state() {
    let input = r#"
        handler StateHandler<S>: State<S> {
            var state: S
            let get = fn () => self.state
            let set = fn (new_state: S) => { self.state = new_state }
        }
    "#;
     let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();

    assert_eq!(program.declarations.len(), 1);
     match &program.declarations[0] {
        Decl::HandlerDecl(decl) => {
            assert_eq!(decl.name, "StateHandler");
            assert!(decl.generic_params.is_some());
            assert_eq!(decl.generic_params.as_ref().unwrap().len(), 1);
            assert_eq!(decl.generic_params.as_ref().unwrap()[0].name, "S");
            assert_eq!(decl.members.len(), 3); // var state, let get, let set

            match &decl.members[0] {
                HandlerMember::Field(f) => {
                    assert!(f.is_mutable);
                    assert_eq!(f.name, "state");
                 match &f.type_annotation {
                        Type::Simple{ name, .. } => assert_eq!(name, "S"),
                        _ => panic!("Expected simple type S for state"),
                    }
                }
                _ => panic!("Expected Field member for state"),
            }
             match &decl.members[1] {
                HandlerMember::Function(f) => assert_eq!(f.name, "get"),
                _ => panic!("Expected Function member for get"),
            }
             match &decl.members[2] { // Assertion for 'set'
                HandlerMember::Function(f) => assert_eq!(f.name, "set"),
                _ => panic!("Expected Function member for set"),
             }
        }
        _ => panic!("Expected HandlerDecl"),
    }
}

#[test]
fn test_parse_handler_with_resume_noresume() {
    let input = r#"
        handler AsyncHandler: Async {
            let await = fn (promise: Promise<T>) resume : R => promise.then(resume) // カンマとResumeTypeを削除し、戻り値型をresumeの後ろに移動
            let fail = fn (err: Error) noresume : Void => raise(err) // コロンをnoresumeの後ろに移動
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();

    assert_eq!(program.declarations.len(), 1);
    match &program.declarations[0] {
        Decl::HandlerDecl(decl) => {
            assert_eq!(decl.name, "AsyncHandler");
            assert_eq!(decl.members.len(), 2);

            // Test 'await' function (resume)
            match &decl.members[0] {
                HandlerMember::Function(f) => {
                    assert_eq!(f.name, "await");
                    match &f.body {
                        HandlerFunctionBody::ResumeFunction(rf) => {
                            assert_eq!(rf.parameters.len(), 1);
                            assert_eq!(rf.parameters[0].name, "promise");
                            // TODO: Add more detailed checks for resume_type and return_type if needed
                        }
                        _ => panic!("Expected ResumeFunction body for 'await'"),
                    }
                }
                _ => panic!("Expected Function member for 'await'"),
            }

            // Test 'fail' function (noresume)
            match &decl.members[1] {
                HandlerMember::Function(f) => {
                    assert_eq!(f.name, "fail");
                    match &f.body {
                        HandlerFunctionBody::NoResumeFunction(nrf) => {
                            assert_eq!(nrf.parameters.len(), 1);
                            assert_eq!(nrf.parameters[0].name, "err");
                            assert!(nrf.return_type.is_some()); // 戻り値型が存在することを確認
                            match nrf.return_type.as_ref().unwrap() { // Option をアンラップして比較
                                Type::Simple { name, .. } => assert_eq!(name, "Void"),
                                _ => panic!("Expected Void return type for 'fail'"),
                            }
                        }
                         _ => panic!("Expected NoResumeFunction body for 'fail'"),
                    }
                }
                _ => panic!("Expected Function member for 'fail'"),
            }
        }
        _ => panic!("Expected HandlerDecl"),
    }
}
