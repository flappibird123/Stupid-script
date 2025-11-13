mod lexer {
    #[derive(Debug)]
    pub struct Token {
        pub kind: TokenKind,
        pub value: Option<String>,
        pub line: usize,
        pub column: usize,
    }

    #[derive(Debug)]
    pub enum TokenKind {
        Keyword(Keyword),
        Identifier(String),
        Operator(Operator),
        Symbol(Symbol),
    }

    #[derive(Debug)]
    pub enum Keyword {
        Print,
        Println,
        Let,
        Const,
    }

    #[derive(Debug)]
    pub enum Operator {
        Assignment, // '='
        Plus,       // '+'
        Minus,      // '-'
        Multiply,   // '*'
        Division,   // '/'
    }

    #[derive(Debug)]
    pub enum Symbol {
        SemiColon, // ';'
        LParen,    // '('
        RParen,    // ')'
    }

    #[derive(Debug)]
    pub enum Type {
        Boolean,
        Int,
        Float,
        Char,
        Double,
        String,
    }

    pub fn lexer(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = text.chars().peekable();

        let mut line = 1;
        let mut column = 1;

        while let Some(&ch) = chars.peek() {

            // Skip whitespace
            if ch.is_whitespace() {
                if ch == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                chars.next();
                continue;
            }

            // Symbols
            let start_col = column;
            match ch {
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Symbol(Symbol::SemiColon),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::Symbol(Symbol::LParen),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::Symbol(Symbol::RParen),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                _ => {}
            }

            // Operators
            match ch {
                '=' => {
                    tokens.push(Token {
                        kind: TokenKind::Operator(Operator::Assignment),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                '+' => {
                    tokens.push(Token {
                        kind: TokenKind::Operator(Operator::Plus),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                '-' => {
                    tokens.push(Token {
                        kind: TokenKind::Operator(Operator::Minus),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                '*' => {
                    tokens.push(Token {
                        kind: TokenKind::Operator(Operator::Multiply),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                '/' => {
                    tokens.push(Token {
                        kind: TokenKind::Operator(Operator::Division),
                        value: None,
                        line,
                        column,
                    });
                    chars.next();
                    column += 1;
                    continue;
                }
                _ => {}
            }

            // Identifiers or keywords
            if ch.is_alphabetic() {
                let mut ident = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        ident.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                let keyword = match ident.as_str() {
                    "print" => Some(Keyword::Print),
                    "println" => Some(Keyword::Println),
                    "let" => Some(Keyword::Let),
                    "const" => Some(Keyword::Const),
                    _ => None,
                };

                if let Some(kw) = keyword {
                    tokens.push(Token {
                        kind: TokenKind::Keyword(kw),
                        value: None,
                        line,
                        column: start_col,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Identifier(ident.clone()),
                        value: Some(ident),
                        line,
                        column: start_col,
                    });
                }

                continue;
            }

            // Unknown character
            chars.next();
            column += 1;
        }

        tokens
    }
}
