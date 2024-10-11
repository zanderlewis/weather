use std::env;
use std::fs;

/* ==== + ==== */
mod token;     //
mod ast;       //
mod constants; //
/* ==== + ==== */

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

mod interpreter;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <script>", args[0]);
        return;
    }

    let script = fs::read_to_string(&args[1]).expect("Failed to read script");
    let lexer = Lexer::new(script);
    let mut parser = Parser::new(lexer);
    let nodes = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(nodes);
}