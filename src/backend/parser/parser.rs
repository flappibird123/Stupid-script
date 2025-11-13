use crate::lexer::{Keyword, Operator, Symbol, Token, TokenKind};
use crate::ast::{Expr, Stmt};

/// A simple recursive-descent parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse all statements in the file
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_end() {
            statements.push(self.parse_statement());
        }

        statements
    }

    // --------------------------
    // TOKEN HELPERS
    // --------------------------

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if !self.is_end() {
            self.pos += 1;
        }
    }

    fn is_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn matches(&mut self, kind: &TokenKind) -> bool {
        if self.is_end() {
            return false;
        }

        let ok = std::mem::discriminant(&self.current().kind)
            == std::mem::discriminant(kind);

        if ok {
            self.advance();
        }

        ok
    }

    // --------------------------
    // STATEMENTS
    // --------------------------

    fn parse_statement(&mut self) -> Stmt {
        match &self.current().kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_var_decl(false),
            TokenKind::Keyword(Keyword::Const) => self.parse_var_decl(true),
            TokenKind::Keyword(Keyword::Print) => self.parse_print(false),
            TokenKind::Keyword(Keyword::Println) => self.parse_print(true),
            _ => panic!("Unexpected statement at line {}", self.current().line),
        }
    }

    fn parse_var_decl(&mut self, constant: bool) -> Stmt {
        self.advance(); // consume `let` or `const`

        // expect identifier
        let name = if let TokenKind::Identifier(n) = &self.current().kind {
            let val = n.clone();
            self.advance();
            val
        } else {
            panic!("Expected identifier after let/const");
        };

        // expect `=`
        match self.current().kind {
            TokenKind::Operator(Operator::Assignment) => self.advance(),
            _ => panic!("Expected '=' after variable name"),
        };

        let expr = self.parse_expression();

        // expect semicolon
        match self.current().kind {
            TokenKind::Symbol(Symbol::SemiColon) => self.advance(),
            _ => panic!("Expected ';' after expression"),
        };

        Stmt::VarDeclaration {
            constant,
            name,
            value: expr,
        }
    }

    fn parse_print(&mut self, newline: bool) -> Stmt {
        self.advance(); // consume print or println

        // expect "("
        match self.current().kind {
            TokenKind::Symbol(Symbol::LParen) => self.advance(),
            _ => panic!("Expected '(' after print"),
        };

        let expr = self.parse_expression();

        // expect ")"
        match self.current().kind {
            TokenKind::Symbol(Symbol::RParen) => self.advance(),
            _ => panic!("Expected ')' after print expression"),
        };

        // optional semicolon
        if let TokenKind::Symbol(Symbol::SemiColon) = self.current().kind {
            self.advance();
        }

        Stmt::Print { newline, expr }
    }

    // --------------------------
    // EXPRESSIONS
    // --------------------------

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        loop {
            match &self.current().kind {
                TokenKind::Operator(Operator::Plus) |
                TokenKind::Operator(Operator::Minus) => {
                    let op = if let TokenKind::Operator(op) = &self.current().kind {
                        op.clone()
                    } else { unreachable!() };

                    self.advance();
                    let right = self.parse_factor();

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match &self.current().kind {
                TokenKind::Operator(Operator::Multiply) |
                TokenKind::Operator(Operator::Division) => {
                    let op = if let TokenKind::Operator(op) = &self.current().kind {
                        op.clone()
                    } else { unreachable!() };

                    self.advance();
                    let right = self.parse_primary();

                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        let tok = self.current().clone();

        match tok.kind {
            TokenKind::Identifier(name) => {
                self.advance();
                Expr::Identifier(name)
            }

            // "hello"
            TokenKind::Symbol(Symbol::DblQuote) => {
                self.advance(); // consume first "

                let mut s = String::new();
                while let TokenKind::Identifier(ch) = &self.current().kind {
                    s.push_str(ch);
                    self.advance();
                }

                // expect closing "
                match self.current().kind {
                    TokenKind::Symbol(Symbol::DblQuote) => self.advance(),
                    _ => panic!("Unclosed string literal"),
                }

                Expr::StringLiteral(s)
            }

            _ => panic!("Unexpected token {:?} in expression", tok.kind),
        }
    }
}
