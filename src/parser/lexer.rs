struct Token {
    kind: TokenKind,
    value: Option<String>,
    line: usize,
    column: usize,
}

enum TokenKind {
    KeyWord(String),
    Identifier(String),
    Type(Type),
    Operator(Operator),
    Literal(String),
    Symbol(Symbol),
}

enum Type {
    Int,
    Float,
    Char,
    Boolean,
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Division,
    Assignment,
}

enum KeyWord {
    Let,
    Const,
    Print,
}

enum Symbol {
    LParen,
    RParen,
    LDblQuote,
    RDblQuote,
    SemiColon,
}


