use std::{env, fs};

mod eval;
mod lexer;
mod node;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dbga = cfg!(debug_assertions);

    if args.len() == 2 && dbga {
        // File path is specified
        let path = &args[1];
        let source = fs::read_to_string(&path).expect("Error reading source file");

        // Create lexer and tokenize
        let mut lexer = lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();

        // Create parser and parse
        let mut parser = parser::Parser::new(tokens);
        let ast = parser.parse_stream();
        dbg!(&ast);

        // Create evaluator
        let mut evaluator = eval::Evaluator::new();
        evaluator.eval(ast);
        dbg!(&evaluator);

        dbg!(&tokens);
    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
