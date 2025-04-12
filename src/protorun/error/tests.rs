// エラーモジュールのテスト

use super::*;
use crate::protorun::ast::Span;

#[test]
fn test_error_syntax() {
    let span = Some(Span {
        start: 20,
        end: 25,
        line: 3,
        column: 10,
    });
    
    let filename = Some("test.pr".to_string());
    let message = "構文エラー";
    
    let error = Error::syntax(message, span.clone(), filename.clone());
    
    assert!(matches!(error.kind, ErrorKind::Syntax(_)));
    match error.kind {
        ErrorKind::Syntax(msg) => assert_eq!(msg, message),
        _ => panic!("期待される構文解析エラーではありません"),
    }
    
    assert_eq!(error.span, span);
    assert_eq!(error.filename, filename);
}

#[test]
fn test_error_other() {
    let span = Some(Span {
        start: 50,
        end: 55,
        line: 6,
        column: 25,
    });
    
    let filename = Some("test.pr".to_string());
    let message = "その他のエラー";
    
    let error = Error::other(message, span.clone(), filename.clone());
    
    assert!(matches!(error.kind, ErrorKind::Other(_)));
    match error.kind {
        ErrorKind::Other(msg) => assert_eq!(msg, message),
        _ => panic!("期待されるその他のエラーではありません"),
    }
    
    assert_eq!(error.span, span);
    assert_eq!(error.filename, filename);
}

#[test]
fn test_error_display_with_filename_and_span() {
    let span = Some(Span {
        start: 10,
        end: 15,
        line: 2,
        column: 5,
    });
    
    let filename = Some("test.pr".to_string());
    let message = "エラーテスト";
    
    let error = Error::syntax(message, span, filename);
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("構文解析エラー"));
    assert!(display_str.contains("エラーテスト"));
    assert!(display_str.contains("test.pr"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("5"));
}

#[test]
fn test_error_display_with_span_only() {
    let span = Some(Span {
        start: 10,
        end: 15,
        line: 2,
        column: 5,
    });
    
    let message = "エラーテスト";
    
    let error = Error::syntax(message, span, None);
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("構文解析エラー"));
    assert!(display_str.contains("エラーテスト"));
    assert!(display_str.contains("行 2"));
    assert!(display_str.contains("列 5"));
    assert!(!display_str.contains("test.pr"));
}

#[test]
fn test_error_display_without_span() {
    let message = "エラーテスト";
    
    let error = Error::syntax(message, None, None);
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("構文解析エラー"));
    assert!(display_str.contains("エラーテスト"));
    assert!(!display_str.contains("行"));
    assert!(!display_str.contains("列"));
}

#[test]
fn test_error_equality() {
    let span1 = Some(Span {
        start: 10,
        end: 15,
        line: 2,
        column: 5,
    });
    
    let span2 = Some(Span {
        start: 10,
        end: 15,
        line: 2,
        column: 5,
    });
    
    let span3 = Some(Span {
        start: 20,
        end: 25,
        line: 3,
        column: 10,
    });
    
    let filename = Some("test.pr".to_string());
    let message = "エラーテスト";
    
    let error1 = Error::syntax(message, span1, filename.clone());
    let error2 = Error::syntax(message, span2, filename.clone());
    let error3 = Error::syntax(message, span3, filename);
    
    assert_eq!(error1, error2); // 同じエラー
    assert_ne!(error1, error3); // 異なるスパン
}

#[test]
fn test_result_type() {
    let span = Some(Span {
        start: 10,
        end: 15,
        line: 2,
        column: 5,
    });
    
    let error = Error::syntax("エラー", span, None);
    
    let result: Result<i32> = Err(error.clone());
    
    match result {
        Ok(_) => panic!("Resultがエラーを返すべきです"),
        Err(e) => assert_eq!(e, error),
    }
    
    let success_result: Result<i32> = Ok(42);
    
    match success_result {
        Ok(value) => assert_eq!(value, 42),
        Err(_) => panic!("Resultが成功値を返すべきです"),
    }
}
