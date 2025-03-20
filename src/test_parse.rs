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
    println!("文数: {}", program.statements.len());
    
    println!("\n関数宣言:");
    for (i, decl) in program.declarations.iter().enumerate() {
        match decl {
            Decl::Function { name, parameters, return_type, .. } => {
                let params: Vec<String> = parameters.iter()
                    .map(|p| {
                        if let Some(t) = &p.type_annotation {
                            match t {
                                Type::Simple { name, .. } => {
                                    format!("{}: {}", p.name, name)
                                },
                                _ => format!("{}: <複合型>", p.name),
                            }
                        } else {
                            p.name.clone()
                        }
                    })
                    .collect();
                
                let ret_type = if let Some(t) = return_type {
                    match t {
                        Type::Simple { name, .. } => {
                            format!(": {}", name)
                        },
                        _ => ": <複合型>".to_string(),
                    }
                } else {
                    String::new()
                };
                
                println!("  関数 #{}: fn {}({}){}", i + 1, name, params.join(", "), ret_type);
            }
        }
    }
    
    println!("\nテスト完了");
}
