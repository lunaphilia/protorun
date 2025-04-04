// Protorun言語のAST定義

/// ソースコード内の位置情報
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,  // 開始位置（文字インデックス）
    pub end: usize,    // 終了位置（文字インデックス）
    pub line: usize,   // 行番号（1始まり）
    pub column: usize, // 列番号（1始まり）
}

/// リテラルパターンの値
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
}

/// パターンマッチングのAST
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// リテラルパターン
    Literal(LiteralValue, Span),
    /// 識別子パターン
    Identifier(String, Span),
    /// タプルパターン
    Tuple(Vec<Pattern>, Span),
    /// コンストラクタパターン
    Constructor {
        name: String,
        arguments: Vec<Pattern>,
        span: Span,
    },
    /// ワイルドカードパターン
    Wildcard(Span),
}

/// コレクション内包表記の種類
#[derive(Debug, Clone, PartialEq)]
pub enum ComprehensionKind {
    List,
    Map,
    Set,
}

// HandlerSpec enum を削除

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
    /// リストリテラル
    ListLiteral {
        elements: Vec<Expr>,
        span: Span,
    },
    /// マップリテラル
    MapLiteral {
        entries: Vec<(Expr, Expr)>,
        span: Span,
    },
    /// セットリテラル
    SetLiteral {
        elements: Vec<Expr>,
        span: Span,
    },
    /// ラムダ式
    LambdaExpr {
        parameters: Vec<Parameter>,
        body: Box<Expr>,
        span: Span,
    },
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
    /// メンバーアクセス
    MemberAccess {
        object: Box<Expr>,
        member: String,
        span: Span,
    },
    /// カッコで囲まれた式
    ParenExpr(Box<Expr>, Span),
    /// if式
    IfExpr {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
        span: Span,
    },
    /// match式
    MatchExpr {
        scrutinee: Box<Expr>,
        cases: Vec<(Pattern, Option<Expr>, Expr)>,
        span: Span,
    },
    /// コレクション内包表記
    CollectionComprehension {
        kind: ComprehensionKind,
        output_expr: Box<Expr>,
        input_expr: Box<Expr>,
        pattern: Pattern,
        condition: Option<Box<Expr>>,
        span: Span,
    },
    /// bind式
    BindExpr {
        bindings: Vec<(Pattern, Expr)>,
        final_expr: Box<Expr>,
        span: Span,
    },
    /// with式
    WithExpr {
        handler: Box<Expr>, // HandlerSpec から Box<Expr> に変更
        effect_type: Option<Type>,
        body: Box<Expr>,
        span: Span,
    },
    /// ブロック式
    BlockExpr {
        items: Vec<BlockItem>, // 宣言、文、または式のリスト
        // final_expr フィールドを削除
        span: Span,
    },
}

/// ブロック内の要素（宣言、文、または式）
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Declaration(Decl),
    Statement(Stmt), // Stmt は Return のみ
    Expression(Expr),  // Expression バリアントを追加
}

/// 文のAST
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Let と Var は Decl に移動
    /// return文
    Return {
        value: Option<Expr>,
        span: Span,
    },
    // Expr バリアントを削除
}

/// 宣言のAST
#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    /// 関数宣言
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Expr, // 関数本体は式
        span: Span,
    },
    /// let宣言 (不変束縛)
    Let {
        pattern: Pattern, // name: String から Pattern に変更
        type_annotation: Option<Type>,
        value: Expr,
        span: Span,
    },
    /// var宣言 (可変変数)
    Var {
        name: String, // var は Identifier のみなので String のまま
        type_annotation: Option<Type>,
        value: Expr,
        span: Span,
    },
    // TODO: 他の宣言タイプ (Type, Trait, Impl, Effect, Handler, Export, Enum) も
    // ここに追加するか、Program 構造体で別々に管理するか検討が必要。
    // 今回はまず Let/Var の移動に集中する。
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

/// enum型のバリアント
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
    pub span: Span,
}

/// 型宣言のAST
#[derive(Debug, Clone, PartialEq)]
pub enum TypeDecl {
    /// レコード型宣言
    Record {
        name: String,
        type_parameters: Vec<String>,
        fields: Vec<(String, Type)>,
        span: Span,
    },
    /// 代数的データ型（enum）宣言
    Enum {
        name: String,
        type_parameters: Vec<String>,
        variants: Vec<EnumVariant>,
        span: Span,
    },
    /// 型エイリアス
    Alias {
        name: String,
        type_parameters: Vec<String>,
        aliased_type: Type,
        span: Span,
    },
}

/// トレイト宣言のAST
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub super_trait: Option<Type>,
    pub methods: Vec<Decl>,
    pub span: Span,
}

/// トレイト実装のAST
#[derive(Debug, Clone, PartialEq)]
pub struct ImplDecl {
    pub type_parameters: Vec<String>,
    pub target_type: Type,
    pub trait_type: Type,
    pub methods: Vec<Decl>,
    pub span: Span,
}

/// エクスポート宣言
#[derive(Debug, Clone, PartialEq)]
pub enum ExportDecl {
    /// 個別エクスポート
    Single {
        name: String,
        span: Span,
    },
    /// グループエクスポート
    Group {
        names: Vec<String>,
        span: Span,
    }
}

/// インポート宣言のアイテム
#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// インポート宣言
#[derive(Debug, Clone, PartialEq)]
pub enum ImportDecl {
    /// 選択的インポート
    Selective {
        module_path: String,
        imports: Vec<ImportItem>,
        span: Span,
    },
    /// モジュール全体のインポート
    Module {
        module_path: String,
        alias: String,
        span: Span,
    }
}

/// モジュール定義
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub path: String,
    pub exports: Vec<ExportDecl>,
    pub imports: Vec<ImportDecl>,
    pub declarations: Vec<Decl>,
    pub type_declarations: Vec<TypeDecl>,
    pub trait_declarations: Vec<TraitDecl>,
    pub impl_declarations: Vec<ImplDecl>,
    // pub statements: Vec<Stmt>, // 削除
    pub expressions: Vec<Expr>, // 追加
    pub span: Span,
}

/// プログラム全体
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub modules: Vec<Module>,
    // declarations には Function, Let, Var が入るようになる
    pub declarations: Vec<Decl>,
    pub type_declarations: Vec<TypeDecl>, // TypeDecl は Decl に含めず分離したままにする
    pub trait_declarations: Vec<TraitDecl>, // TraitDecl も分離
    pub impl_declarations: Vec<ImplDecl>,   // ImplDecl も分離
    // statements には Return のみが含まれるはずだが、トップレベルには書けない
    // pub statements: Vec<Stmt>, // 削除
    // トップレベルの式を保持するフィールドを追加
    pub expressions: Vec<Expr>,
}

#[cfg(test)]
mod tests;
