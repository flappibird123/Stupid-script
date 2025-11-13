use std::fmt;

/// Token types for the Stupid Script language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),

    // Keywords
    Let,
    Const,
    If,
    Else,
    For,
    While,
    Fn,
    Return,
    Break,
    Continue,
    Int,
    Float,
    Bool,
    String,
    True,
    False,
    Void,
    Print,
    Println,
    Null,

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Arrow,
    FatArrow,

    // Special
    Eof,
}

/// Token structure containing type and position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token({:?}) at line: {}, column: {}",
            self.token_type, self.line, self.column
        )
    }
}

/// Lexer for tokenizing Stupid Script source code
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Peek the current character without consuming it
    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    /// Peek the next character without consuming it
    fn peek_next(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    /// Consume and return the current character
    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(ch)
        } else {
            None
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip single-line comments (//)
    fn skip_comment(&mut self) {
        if self.peek() == Some('/') && self.peek_next() == Some('/') {
            self.advance(); // skip first /
            self.advance(); // skip second /
            while let Some(ch) = self.peek() {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    /// Skip multi-line comments (/* */)
    fn skip_multi_comment(&mut self) {
        if self.peek() == Some('/') && self.peek_next() == Some('*') {
            self.advance(); // skip /
            self.advance(); // skip *
            while let Some(ch) = self.peek() {
                if ch == '*' && self.peek_next() == Some('/') {
                    self.advance(); // skip *
                    self.advance(); // skip /
                    break;
                }
                self.advance();
            }
        }
    }

    /// Read a string literal
    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.peek() {
            if ch == quote {
                self.advance(); // skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.peek() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        result
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> TokenType {
        let mut number = String::new();
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !is_float && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            TokenType::Float(number.parse().unwrap_or(0.0))
        } else {
            TokenType::Integer(number.parse().unwrap_or(0))
        }
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    /// Check if an identifier is a keyword
    fn keyword_or_identifier(&self, ident: &str) -> TokenType {
        match ident {
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "string" => TokenType::String,
            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),
            "void" => TokenType::Void,
            "print" => TokenType::Print,
            "println" => TokenType::Println,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();

            // Handle comments
            if self.peek() == Some('/') {
                if self.peek_next() == Some('/') {
                    self.skip_comment();
                    continue;
                } else if self.peek_next() == Some('*') {
                    self.skip_multi_comment();
                    continue;
                }
            }

            break;
        }

        let line = self.line;
        let column = self.column;

        match self.peek() {
            None => Token::new(TokenType::Eof, line, column),

            Some('"') => {
                let string = self.read_string('"');
                Token::new(TokenType::String(string), line, column)
            }

            Some('\'') => {
                let string = self.read_string('\'');
                Token::new(TokenType::String(string), line, column)
            }

            Some(ch) if ch.is_ascii_digit() => {
                let token_type = self.read_number();
                Token::new(token_type, line, column)
            }

            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                let token_type = self.keyword_or_identifier(&ident);
                Token::new(token_type, line, column)
            }

            Some('+') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::PlusEqual, line, column)
                } else {
                    Token::new(TokenType::Plus, line, column)
                }
            }

            Some('-') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::MinusEqual, line, column)
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::new(TokenType::Arrow, line, column)
                } else {
                    Token::new(TokenType::Minus, line, column)
                }
            }

            Some('*') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::StarEqual, line, column)
                } else {
                    Token::new(TokenType::Star, line, column)
                }
            }

            Some('/') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::SlashEqual, line, column)
                } else {
                    Token::new(TokenType::Slash, line, column)
                }
            }

            Some('%') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::PercentEqual, line, column)
                } else {
                    Token::new(TokenType::Percent, line, column)
                }
            }

            Some('=') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::EqualEqual, line, column)
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::new(TokenType::FatArrow, line, column)
                } else {
                    Token::new(TokenType::Equal, line, column)
                }
            }

            Some('!') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::NotEqual, line, column)
                } else {
                    Token::new(TokenType::Not, line, column)
                }
            }

            Some('<') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::LessEqual, line, column)
                } else if self.peek() == Some('<') {
                    self.advance();
                    Token::new(TokenType::LeftShift, line, column)
                } else {
                    Token::new(TokenType::Less, line, column)
                }
            }

            Some('>') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::new(TokenType::GreaterEqual, line, column)
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::new(TokenType::RightShift, line, column)
                } else {
                    Token::new(TokenType::Greater, line, column)
                }
            }

            Some('&') => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    Token::new(TokenType::And, line, column)
                } else {
                    Token::new(TokenType::BitwiseAnd, line, column)
                }
            }

            Some('|') => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    Token::new(TokenType::Or, line, column)
                } else {
                    Token::new(TokenType::BitwiseOr, line, column)
                }
            }

            Some('^') => {
                self.advance();
                Token::new(TokenType::BitwiseXor, line, column)
            }

            Some('~') => {
                self.advance();
                Token::new(TokenType::BitwiseNot, line, column)
            }

            Some('(') => {
                self.advance();
                Token::new(TokenType::LeftParen, line, column)
            }

            Some(')') => {
                self.advance();
                Token::new(TokenType::RightParen, line, column)
            }

            Some('{') => {
                self.advance();
                Token::new(TokenType::LeftBrace, line, column)
            }

            Some('}') => {
                self.advance();
                Token::new(TokenType::RightBrace, line, column)
            }

            Some('[') => {
                self.advance();
                Token::new(TokenType::LeftBracket, line, column)
            }

            Some(']') => {
                self.advance();
                Token::new(TokenType::RightBracket, line, column)
            }

            Some(';') => {
                self.advance();
                Token::new(TokenType::Semicolon, line, column)
            }

            Some(':') => {
                self.advance();
                Token::new(TokenType::Colon, line, column)
            }

            Some(',') => {
                self.advance();
                Token::new(TokenType::Comma, line, column)
            }

            Some('.') => {
                self.advance();
                Token::new(TokenType::Dot, line, column)
            }

            Some(ch) => {
                // Skip unknown characters
                self.advance();
                self.next_token()
            }
        }
    }

    /// Tokenize the entire input and return a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let const if else for while fn return");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Const));
        assert!(matches!(tokens[2].token_type, TokenType::If));
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("x variable_name _private");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Identifier(_)));
    }

    #[test]
    fn test_integers() {
        let mut lexer = Lexer::new("42 0 999");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Integer(42)));
        assert!(matches!(tokens[1].token_type, TokenType::Integer(0)));
        assert!(matches!(tokens[2].token_type, TokenType::Integer(999)));
    }

    #[test]
    fn test_floats() {
        let mut lexer = Lexer::new("3.14 0.5 10.0");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Float(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Float(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Float(_)));
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello world" 'test'"#);
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::String(_)));
        assert!(matches!(tokens[1].token_type, TokenType::String(_)));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / % == != < > <= >=");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Plus));
        assert!(matches!(tokens[1].token_type, TokenType::Minus));
        assert!(matches!(tokens[4].token_type, TokenType::Percent));
        assert!(matches!(tokens[5].token_type, TokenType::EqualEqual));
    }

    #[test]
    fn test_delimiters() {
        let mut lexer = Lexer::new("( ) { } [ ] ; : , .");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::LeftParen));
        assert!(matches!(tokens[1].token_type, TokenType::RightParen));
        assert!(matches!(tokens[2].token_type, TokenType::LeftBrace));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("let x = 5; // this is a comment\nlet y = 10;");
        let tokens = lexer.tokenize();
        // Comments should be skipped
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
    }

    #[test]
    fn test_multiline_comments() {
        let mut lexer = Lexer::new("let x = 5; /* this is a comment */ let y = 10;");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].token_type, TokenType::Let));
    }

    #[test]
    fn test_example_program() {
        let code = r#"let int x = 5;
const int y = 10;

if (x < y) {
    println("x is less than y");
}"#;
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        assert!(!tokens.is_empty());
        assert!(matches!(tokens[tokens.len() - 1].token_type, TokenType::Eof));
    }
}
