use std::fmt;

mod lexer {
    pub struct Token {
        kind: TokenKind,
        value: Option<String>,
        line: usize,
        column: usize,
    }
    enum TokenKind {
        Keyword(Keyword),
        Identifier(String),
        Operator(Operator),
        Symbol(Symbol),
    }
    enum Keyword {
        Print,
        Println,
        Let,
        Const,
    }
    enum Operator {
        Assignment,
        Plus,
        Minus,
        Multply,
        Division,
    }
    enum Symbol {
        SemiColon,
        LParen,
        RParen,
    }
    enum Type {
        Boolean,
        Int,
        Float,
        Char,
        Double,
        String,
    }

    fn lexer(text) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens
    }
}
