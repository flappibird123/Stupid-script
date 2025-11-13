mod lexer;
mod ast;
mod parser;
mod runtime;

use crate::lexer::lexer as lex;
use crate::parser::Parser;
use crate::runtime::Interpreter;

fn main() {
    let source = r#"
        let x = 3;
        let y = 4;
        let s = "hello";
        print(s);
        println(" world");
        println(x + y);
        println(s + " world");
    "#;

    // 1) Lex
    let tokens = lex(source);

    // 2) Parse
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    // 3) Interpret
    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(stmts) {
        eprintln!("Runtime error: {:?}", e);
    }
}
