// Protorun言語の構文解析器 - モジュラー版

// サブモジュールの公開
pub mod common;
pub mod literals;
pub mod patterns;
pub mod types;
pub mod statements;
pub mod expressions;

use nom::Finish;

use super::ast::{Expr, Program, Type};
use super::error::Result;
use common::{ParserContext, to_syntax_error};

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
        let ctx = ParserContext::new(input, self.filename.clone());
        match statements::program(input, &ctx).finish() {
            Ok((_, program)) => Ok(program),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }

    /// 式をパース
    pub fn parse_expression(&mut self, input: &str) -> Result<Expr> {
        let ctx = ParserContext::new(input, self.filename.clone());
        match expressions::expression(input, &ctx).finish() {
            Ok((_, expr)) => Ok(expr),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
    
    /// 型をパース
    pub fn parse_type(&mut self, input: &str) -> Result<Type> {
        let ctx = ParserContext::new(input, self.filename.clone());
        match types::type_parser(input, &ctx).finish() {
            Ok((_, ty)) => Ok(ty),
            Err(error) => Err(to_syntax_error(input, error, self.filename.clone())),
        }
    }
}

#[cfg(test)]
mod tests;
