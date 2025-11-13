use crate::ast::Expr;
use crate::lexer::Keyword;

/// Top-level statement nodes
#[derive(Debug)]
pub enum Stmt {
    /// let x = 10;
    VarDeclaration {
        constant: bool,
        name: String,
        value: Expr,
    },

    /// print(expr);
    Print {
        newline: bool, // true = println
        expr: Expr,
    },
}
