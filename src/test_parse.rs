// 構文解析のテスト用スクリプト

use std::fs;

// protorunモジュールをインポート
mod protorun;
use protorun::parser::Parser;
use protorun::ast::{Decl, Type};

fn main() {
    println!("Protorunの構文解析器テスト");
    
    // サンプルファイルのパス
    let sample_path = "examples/hello.pr";
    
    // ファイルを読み込む（UTF-8として正しく処理する）
    let content = match fs::read_to_string(sample_path) {
        Ok(content) => {
            // 改行コードを統一（Windows環境で問題が発生する可能性があるため）
            content.replace("\r\n", "\n")
        },
        Err(e) => {
            eprintln!("ファイルの読み込みエラー: {}", e);
            return;
        }
    };
    
    println!("ファイル内容:");
    println!("{}", content);
    println!();
    
    // 構文解析を行う
    let mut parser = Parser::new(Some(sample_path.to_string()));
    let program = match parser.parse_program(&content) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("構文解析エラー: {}", e);
            return;
        }
    };
    
    println!("解析成功！");
    println!("宣言数: {}", program.declarations.len());
    // println!("文数: {}", program.statements.len()); // 削除
    println!("トップレベル式数: {}", program.expressions.len()); // 追加
    
    println!("\n宣言:"); // タイトル変更
    for (i, decl) in program.declarations.iter().enumerate() {
        match decl {
            // Decl::Function アームを削除
            Decl::Let { pattern, type_annotation, value, .. } => { // value を追加
                // パターンと型注釈（あれば）を表示
                let type_str = if let Some(t) = type_annotation {
                    match t {
                        Type::Simple { name, .. } => format!(": {}", name),
                        _ => ": <複合型>".to_string(),
                    }
                } else {
                    String::new()
                };
                // パターンを単純化して表示（ここでは識別子のみ想定）
                let pattern_str = match pattern {
                    protorun::ast::Pattern::Identifier(name, _) => name.clone(),
                    _ => "<パターン>".to_string(),
                };
                // 値がラムダ式の場合、関数として表示する（簡易版）
                if let protorun::ast::Expr::LambdaExpr { parameters, .. } = value {
                    let params_str = parameters.as_ref().map_or("()".to_string(), |params| {
                        let p_strs: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                        format!("({})", p_strs.join(", "))
                    });
                    println!("  関数 (Let) #{}: let {}{} = fn ...", i + 1, pattern_str, type_str);
                } else {
                    println!("  Let宣言 #{}: let {}{}", i + 1, pattern_str, type_str);
                }
            },
            Decl::Var { name, type_annotation, .. } => {
                // 変数名と型注釈（あれば）を表示
                 let type_str = if let Some(t) = type_annotation {
                    match t {
                        Type::Simple { name, .. } => format!(": {}", name),
                        _ => ": <複合型>".to_string(),
                    }
                } else {
                    String::new()
                };
                println!("  Var宣言 #{}: var {}{}", i + 1, name, type_str);
            },
        }
    }
    
    println!("\nテスト完了");
}
