// 宣言パーサーのテスト

use super::*;
use crate::protorun::ast::{TypeDecl, EnumVariant, TraitDecl, ImplDecl, Type};

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
            assert_eq!(type_parameters[0], "T");
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
            assert_eq!(type_parameters[0], "T");
            
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
