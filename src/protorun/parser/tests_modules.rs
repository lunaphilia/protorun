// モジュールパーサーのテスト

use super::*;
use crate::protorun::ast::{ExportDecl, ImportDecl};

#[test]
fn test_parse_module() {
    let input = r#"
        module Math {
            export fn add(x: Int, y: Int): Int = x + y
            export fn multiply(x: Int, y: Int): Int = x * y
        }
    "#;
    let mut parser = Parser::new(None);
    let program = parser.parse_program(input).unwrap();
    
    assert_eq!(program.modules.len(), 1);
    
    let module = &program.modules[0];
    assert_eq!(module.path, "Math");
    assert_eq!(module.exports.len(), 2);
    assert_eq!(module.declarations.len(), 2);
}

#[test]
fn test_parse_export() {
    let input = r#"
        module Test {
            export fn test() = 42
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
