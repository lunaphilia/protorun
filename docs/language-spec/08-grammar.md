# 10. 文法（EBNF）

以下に、Protorun言語の主要な構文をEBNF形式で示します。

```ebnf
Program ::= (Declaration | Statement)*

Declaration ::= FunctionDecl | TypeDecl | TraitDecl | ImplDecl | EffectDecl | HandlerDecl

FunctionDecl ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("with" EffectType)? "=" Expression

TypeDecl ::= "type" Identifier GenericParams? "=" (RecordType | Type)
           | "sealed" "trait" Identifier GenericParams? ("{" TraitMember* "}")? ("extends" TypeRef)?
           | "managed" "type" Identifier GenericParams? "{" ManagedTypeMember* "}"
           | "context" "type" Identifier GenericParams? "{" ManagedTypeMember* "}"

TraitDecl ::= "trait" Identifier GenericParams? ("{" TraitMember* "}")? ("extends" TypeRef)?

ImplDecl ::= "impl" GenericParams? TypeRef "for"? TypeRef "{" ImplMember* "}"

EffectDecl ::= "effect" Identifier GenericParams? ("with" "lifecycle")? "{" EffectOperation* "}"

HandlerDecl ::= "handler" Identifier GenericParams? "for" TypeRef "{" HandlerMember* "}"

RecordType ::= "{" (Identifier ":" Type ("," Identifier ":" Type)*)? "}"

TraitMember ::= FunctionDecl

ImplMember ::= FunctionDecl

HandlerMember ::= HandlerFunction | FieldDecl | FinalizeFunction

HandlerFunction ::= Identifier GenericParams? ParamList (":" ReturnType)? "=" Expression
                  | Identifier GenericParams? ParamList "," "resume" ":" ResumeType ":" ReturnType "=" Expression
                  | Identifier GenericParams? ParamList ":" "noresume" ReturnType "=" Expression
                  | Identifier GenericParams? ParamList ":" "multiresume" ReturnType "=" Expression

FinalizeFunction ::= "fn" "finalize" "(" ")" ":" "Unit" "=" Expression

ManagedTypeMember ::= FieldDecl | FunctionDecl

FieldDecl ::= Identifier ":" Type

EffectOperation ::= "fn" Identifier GenericParams? ParamList (":" Type)? ("with" "cleanup")? ";"
                 | "fn" "cleanup" ParamList (":" Type)? ";"

ParamList ::= "(" (Param ("," Param)*)? ")"

Param ::= Identifier ":" Type

GenericParams ::= "<" (GenericParam ("," GenericParam)*)? ">"

GenericParam ::= Identifier (":" TypeConstraint)?

TypeConstraint ::= TypeRef (("&" | "|") TypeRef)*

Type ::= TypeRef
       | FunctionType
       | TupleType
       | ArrayType

TypeRef ::= Identifier GenericArgs?
          | "own" TypeRef

GenericArgs ::= "<" (Type ("," Type)*)? ">"

FunctionType ::= "(" (Type ("," Type)*)? ")" "->" Type ("with" EffectType)?

TupleType ::= "(" Type ("," Type)+ ")"

ArrayType ::= "[" Type "]"

EffectType ::= TypeRef ("&" TypeRef)*

ResumeType ::= "(" (Type ("," Type)*)? ")" "->" ReturnType

ReturnType ::= Type | "Unit"

Statement ::= Expression ";"
            | "let" Pattern (":" Type)? "=" Expression ";"
            | "var" Identifier (":" Type)? "=" Expression ";"
            | "return" Expression? ";"

Expression ::= LiteralExpr
             | IdentifierExpr
             | BlockExpr
             | IfExpr
             | MatchExpr
             | ForExpr
             | LambdaExpr
             | CallExpr
             | MemberAccessExpr
             | BinaryExpr
             | UnaryExpr
             | HandleExpr
             | WithExpr
             | ScopedEffectExpr

LiteralExpr ::= IntLiteral | FloatLiteral | StringLiteral | BoolLiteral | UnitLiteral

IdentifierExpr ::= Identifier

BlockExpr ::= "{" Statement* (Expression)? "}"

IfExpr ::= "if" Expression BlockExpr ("else" (IfExpr | BlockExpr))?

MatchExpr ::= "match" Expression "{" (Pattern ("if" Expression)? "=>" Expression ",")* "}"

ForExpr ::= "for" "{" (Pattern "<-" Expression ("if" Expression)?)* "}" "yield" Expression

LambdaExpr ::= ParamList "=>" Expression

CallExpr ::= Expression "(" (Expression ("," Expression)*)? ")"

MemberAccessExpr ::= Expression "." Identifier

BinaryExpr ::= Expression Operator Expression

UnaryExpr ::= Operator Expression

HandleExpr ::= "handle" Expression "{" (EffectCase)* "}"

WithExpr ::= "with" (Expression | TypeRef) ("handled" "by" Expression)? BlockExpr

ScopedEffectExpr ::= "with" "scoped" "effect" Identifier BlockExpr

EffectCase ::= QualifiedIdentifier ParamList "=>" BlockExpr

Pattern ::= LiteralPattern
          | IdentifierPattern
          | TuplePattern
          | ConstructorPattern
          | WildcardPattern

LiteralPattern ::= LiteralExpr

IdentifierPattern ::= Identifier

TuplePattern ::= "(" Pattern ("," Pattern)* ")"

ConstructorPattern ::= QualifiedIdentifier ("(" Pattern ("," Pattern)* ")")?

WildcardPattern ::= "_"

QualifiedIdentifier ::= (Identifier ".")* Identifier
```

注意: この文法は、新しく追加された`handler`構文や`noresume`、`multiresume`などの特殊な継続制御のための構文を含んでいます。
