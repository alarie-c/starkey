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

        // Create parser and parse
        let mut parser = frontend::parser::Parser::new(tokens.iter());
        parser.parse();
        dbg!(&parser);

    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
