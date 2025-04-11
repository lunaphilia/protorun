// モジュールパーサーのテスト

use super::*;
use crate::protorun::ast::{ExportDecl, ImportDecl, Decl, Pattern, Expr}; // Decl, Pattern, Expr を追加

#[test]
fn test_parse_module() {
    let input = r#"
        module Math {
            export let add = fn (x: Int, y: Int): Int x + y // Comma added between parameters
            export let multiply = fn (x: Int, y: Int): Int x * y // Comma added between parameters
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();

    assert_eq!(program.modules.len(), 1);

    let module = &program.modules[0];
    assert_eq!(module.path, "Math");
    assert_eq!(module.exports.len(), 2); // export let ... は export { ... } と同じ扱いになるはず
    assert_eq!(module.declarations.len(), 2); // let add = ..., let multiply = ...

    // エクスポートされた宣言が正しいか確認
    match &module.exports[0] {
        ExportDecl::Single { name, .. } => assert_eq!(name, "add"),
        _ => panic!("Expected single export for add"),
    }
     match &module.exports[1] {
        ExportDecl::Single { name, .. } => assert_eq!(name, "multiply"),
        _ => panic!("Expected single export for multiply"),
    }

    // 宣言の内容を確認
    match &module.declarations[0] {
        Decl::Let { pattern, value, .. } => {
            match pattern {
                Pattern::Identifier(name, _) => assert_eq!(name, "add"),
                _ => panic!("Expected identifier pattern for add"),
            }
            match value {
                Expr::LambdaExpr { .. } => (), // ラムダ式であることを確認
                _ => panic!("Expected lambda expression for add"),
            }
        },
        _ => panic!("Expected let declaration for add"),
    }
     match &module.declarations[1] {
        Decl::Let { pattern, value, .. } => {
            match pattern {
                Pattern::Identifier(name, _) => assert_eq!(name, "multiply"),
                _ => panic!("Expected identifier pattern for multiply"),
            }
             match value {
                Expr::LambdaExpr { .. } => (), // ラムダ式であることを確認
                _ => panic!("Expected lambda expression for multiply"),
            }
        },
        _ => panic!("Expected let declaration for multiply"),
    }
}

#[test]
fn test_parse_export() {
    let input = r#"
        module Test {
            export let test = fn () 42 // 関数宣言を let + fn に変更
            export {
                add,
                multiply
            }
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    let module = &program.modules[0];
    assert_eq!(module.exports.len(), 2);
    
    match &module.exports[0] {
        ExportDecl::Single { name, .. } => assert_eq!(name, "test"),
        _ => panic!("期待される個別エクスポートではありません"),
    }
    
    match &module.exports[1] {
        ExportDecl::Group { names, .. } => {
            assert_eq!(names.len(), 2);
            assert_eq!(names[0], "add");
            assert_eq!(names[1], "multiply");
        },
        _ => panic!("期待されるグループエクスポートではありません"),
    }
}

#[test]
fn test_parse_import() {
    let input = r#"
        module Test {
            import { add, subtract as sub } from "Math"
            import "Collections" as Col
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    let module = &program.modules[0];
    assert_eq!(module.imports.len(), 2);
    
    match &module.imports[0] {
        ImportDecl::Selective { module_path, imports, .. } => {
            assert_eq!(module_path, "Math");
            assert_eq!(imports.len(), 2);
            
            assert_eq!(imports[0].name, "add");
            assert_eq!(imports[0].alias, None);
            
            assert_eq!(imports[1].name, "subtract");
            assert_eq!(imports[1].alias, Some("sub".to_string()));
        },
        _ => panic!("期待される選択的インポートではありません"),
    }
    
    match &module.imports[1] {
        ImportDecl::Module { module_path, alias, .. } => {
            assert_eq!(module_path, "Collections");
            assert_eq!(alias, "Col");
        },
        _ => panic!("期待されるモジュールインポートではありません"),
    }
}
