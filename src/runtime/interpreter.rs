use crate::ast::{Expr, Stmt};
use crate::lexer::Operator;
use crate::runtime::{Environment, Value};

/// Errors that can happen while interpreting.
#[derive(Debug)]
pub enum RuntimeError {
    Message(String),
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self { RuntimeError::Message(s) }
}
impl From<&str> for RuntimeError {
    fn from(s: &str) -> Self { RuntimeError::Message(s.to_string()) }
}

/// The interpreter. Keeps an environment and executes statements.
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Environment::new() }
    }

    /// Run a sequence of statements.
    pub fn run(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::VarDeclaration { constant, name, value } => {
                let val = self.eval_expr(value)?;
                // if variable exists and is const, Environment::define handles it
                self.env.define(name, val, constant)
                    .map_err(RuntimeError::from)
            }

            Stmt::Print { newline, expr } => {
                let v = self.eval_expr(expr)?;
                if newline {
                    println!("{}", v);
                } else {
                    print!("{}", v);
                }
                Ok(())
            }
        }
    }

    /// Evaluate an expression to a Value.
    fn eval_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::IntLiteral(i) => Ok(Value::Int(i)),
            Expr::StringLiteral(s) => Ok(Value::Str(s)),
            Expr::Identifier(name) => {
                self.env.get(&name)
                    .ok_or_else(|| RuntimeError::Message(format!("Undefined identifier '{}'", name)))
            }
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(*left)?;
                let r = self.eval_expr(*right)?;
                self.apply_binary_op(&l, &op, &r)
            }
        }
    }

    /// Apply binary operator to two values.
    fn apply_binary_op(&self, left: &Value, op: &Operator, right: &Value) -> Result<Value, RuntimeError> {
        use Operator::*;
        match op {
            Plus => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                // allow mixing via tostring
                (a, b) => Ok(Value::Str(format!("{}{}", a.to_string_value(), b.to_string_value()))),
            },

            Minus => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                _ => Err(RuntimeError::Message("'-' operator requires integer operands".into())),
            },

            Multiply => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                _ => Err(RuntimeError::Message("'*' operator requires integer operands".into())),
            },

            Division => match (left, right) {
                (Value::Int(_), Value::Int(0)) => Err(RuntimeError::Message("Division by zero".into())),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                _ => Err(RuntimeError::Message("'/' operator requires integer operands".into())),
            },

            // Assignment operator shouldn't appear as binary expression in our design:
            Assignment => Err(RuntimeError::Message("Unexpected assignment operator in expression".into())),
        }
    }
}
