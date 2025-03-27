// Protorun言語の構文解析器 - モジュラー版

// サブモジュールの公開
pub mod common;
pub mod literals;
pub mod patterns;
pub mod types;
pub mod statements;
pub mod expressions;
pub mod declarations;
pub mod modules;

// 型宣言パーサーをエクスポート

// モジュールパーサーをエクスポート

use nom::Finish;

use super::ast::{Expr, Program, Type};
use super::error::Result;
use common::to_syntax_error;

/// パーサー
pub struct Parser {
    /// ファイル名
    filename: Option<String>,
}

impl Parser {
    /// 新しいパーサーを作成
    pub fn new(filename: Option<String>) -> Self {
        Self { filename }
    }

    /// プログラム全体をパース
    pub fn parse_program(&mut self, input: &str) -> Result<Program> {
        match statements::program(input, input).finish() {
            Ok((_, program)) => Ok(program),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }

    /// 式をパース
    pub fn parse_expression(&mut self, input: &str) -> Result<Expr> {
        match expressions::expression(input, input).finish() {
            Ok((_, expr)) => Ok(expr),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
    
    /// 型をパース
    pub fn parse_type(&mut self, input: &str) -> Result<Type> {
        match types::parse_type(input, input).finish() {
            Ok((_, ty)) => Ok(ty),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
}


#[cfg(test)]
mod tests_scope;
#[cfg(test)]
mod tests_expressions;
#[cfg(test)]
mod tests_statements;
#[cfg(test)]
mod tests_declarations;
#[cfg(test)]
mod tests_modules;
#[cfg(test)]
mod tests_types;
#[cfg(test)]
mod tests_patterns;
#[cfg(test)]
mod tests_literals;
#[cfg(test)]
mod tests_common;
