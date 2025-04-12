// Protorun言語のエラー定義

use std::fmt;
use super::ast::Span;

/// エラーの種類
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // /// 字句解析エラー
    // Lexical(String), // 未使用のためコメントアウト
    /// 構文解析エラー
    Syntax(String),
    // /// 型エラー
    // Type(String), // 未使用のためコメントアウト
    // /// ランタイムエラー
    // Runtime(String), // 未使用のためコメントアウト
    /// その他のエラー
    Other(String),
}

/// エラー情報
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// エラーの種類
    pub kind: ErrorKind,
    /// エラーの位置情報
    pub span: Option<Span>,
    /// ファイル名
    pub filename: Option<String>,
}

impl Error {
    // /// 新しい字句解析エラーを作成
    // pub fn lexical(message: impl Into<String>, span: Option<Span>, filename: Option<String>) -> Self {
    //     Self {
    //         kind: ErrorKind::Lexical(message.into()),
    //         span,
    //         filename,
    //     }
    // }

    /// 新しい構文解析エラーを作成
    pub fn syntax(message: impl Into<String>, span: Option<Span>, filename: Option<String>) -> Self {
        Self {
            kind: ErrorKind::Syntax(message.into()),
            span,
            filename,
        }
    }

    // /// 新しい型エラーを作成
    // pub fn type_error(message: impl Into<String>, span: Option<Span>, filename: Option<String>) -> Self {
    //     Self {
    //         kind: ErrorKind::Type(message.into()),
    //         span,
    //         filename,
    //     }
    // }

    // /// 新しいランタイムエラーを作成
    // pub fn runtime(message: impl Into<String>, span: Option<Span>, filename: Option<String>) -> Self {
    //     Self {
    //         kind: ErrorKind::Runtime(message.into()),
    //         span,
    //         filename,
    //     }
    // }

    /// その他のエラーを作成
    pub fn other(message: impl Into<String>, span: Option<Span>, filename: Option<String>) -> Self {
        Self {
            kind: ErrorKind::Other(message.into()),
            span,
            filename,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            // ErrorKind::Lexical(msg) => write!(f, "字句解析エラー: {}", msg), // コメントアウト
            ErrorKind::Syntax(msg) => write!(f, "構文解析エラー: {}", msg),
            // ErrorKind::Type(msg) => write!(f, "型エラー: {}", msg), // コメントアウト
            // ErrorKind::Runtime(msg) => write!(f, "ランタイムエラー: {}", msg), // コメントアウト
            ErrorKind::Other(msg) => write!(f, "エラー: {}", msg),
        }?;

        if let (Some(span), Some(filename)) = (&self.span, &self.filename) {
            write!(f, " ({}:{}:{})", filename, span.line, span.column)?;
        } else if let Some(span) = &self.span {
            write!(f, " (行 {}、列 {})", span.line, span.column)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

/// Protorun言語の結果型
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
