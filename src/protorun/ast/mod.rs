// Protorun言語のAST定義

/// ソースコード内の位置情報
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,  // 開始位置（文字インデックス）
    pub end: usize,    // 終了位置（文字インデックス）
    pub line: usize,   // 行番号（1始まり）
    pub column: usize, // 列番号（1始まり）
}

/// 式のAST
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// 整数リテラル
    IntLiteral(i64, Span),
    /// 浮動小数点リテラル
    FloatLiteral(f64, Span),
    /// 真偽値リテラル
    BoolLiteral(bool, Span),
    /// 文字列リテラル
    StringLiteral(String, Span),
    /// ユニットリテラル
    UnitLiteral(Span),
    /// 識別子
    Identifier(String, Span),
    /// 二項演算
    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
        span: Span,
    },
    /// 単項演算
    UnaryOp {
        operator: UnaryOperator,
        expr: Box<Expr>,
        span: Span,
    },
    /// 関数呼び出し
    FunctionCall {
        function: Box<Expr>,
        arguments: Vec<Expr>,
        span: Span,
    },
    /// カッコで囲まれた式
    ParenExpr(Box<Expr>, Span),
}

/// 文のAST
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// let宣言文
    Let {
        name: String,
        type_annotation: Option<Type>,
        value: Expr,
        span: Span,
    },
    /// 式文
    Expr {
        expr: Expr,
        span: Span,
    },
}

/// 宣言のAST
#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    /// 関数宣言
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Expr,
        span: Span,
    },
}

/// 関数パラメータ
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub span: Span,
}

/// 型の表現
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// 単純型（Int, Bool, Stringなど）
    Simple {
        name: String,
        span: Span,
    },
    /// 関数型
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
        span: Span,
    },
    /// 配列型
    Array {
        element_type: Box<Type>,
        span: Span,
    },
    /// タプル型
    Tuple {
        element_types: Vec<Type>,
        span: Span,
    },
    /// ジェネリック型
    Generic {
        base_type: String,
        type_arguments: Vec<Type>,
        span: Span,
    },
    /// 参照型
    Reference {
        is_mutable: bool,
        referenced_type: Box<Type>,
        span: Span,
    },
    /// 所有権型
    Owned {
        owned_type: Box<Type>,
        span: Span,
    },
    /// 効果付き型
    WithEffect {
        base_type: Box<Type>,
        effect_type: Box<Type>,
        span: Span,
    },
}

/// 二項演算子
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    // 算術演算子
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    
    // 比較演算子
    Eq,     // ==
    Neq,    // !=
    Lt,     // <
    Gt,     // >
    Lte,    // <=
    Gte,    // >=
    
    // 論理演算子
    And,    // &&
    Or,     // ||
}

/// 単項演算子
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Neg,    // -
    Not,    // !
}

/// プログラム全体
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Decl>,
    pub statements: Vec<Stmt>,
}

#[cfg(test)]
mod tests;
