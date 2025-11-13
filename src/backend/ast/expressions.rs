use crate::lexer::{Operator, Token};

/// Represents all possible expressions in Stupid Script
#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),

    /// String literal: "hello world"
    StringLiteral(String),

    /// Number literal (integer only for now)
    IntLiteral(i64),

    /// Binary operators such as `a + b`
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}
