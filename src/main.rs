use std::{env, fs};

use frontend::parser;

mod backend;
mod frontend;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dbga = cfg!(debug_assertions);

    if args.len() == 2 && dbga {
        // File path is specified
        let path = &args[1];
        let source = fs::read_to_string(&path).expect("Error reading source file");

        // Create lexer and tokenize
        let mut lexer = frontend::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();

        // Create evaluator
        // let mut evaluator = eval::Evaluator::new();
        // evaluator.eval(ast);
        // dbg!(&evaluator);

        let parse_tree = frontend::parser::parse(tokens);
        if parse_tree.is_some() {
            dbg!(&parse_tree.unwrap());
        }

        // dbg!(&tokens);
    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
