use std::{env, fs};

mod errors;
mod frontend;
mod backend;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dbga = cfg!(debug_assertions);

    if args.len() == 2 && dbga {
        // File path is specified
        let path = &args[1];
        let source = fs::read_to_string(&path).expect("Error reading source file");

        // Initialize errors
        let formatter = errors::formatter::Formatter::initialize(&source);
        let mut error_handler = errors::error::Errors::initialize(&formatter);

        // Create lexer and tokenize
        let mut lexer = frontend::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();

        // Create parser and parse
        let mut parser = frontend::parser::Parser::new(&mut error_handler, tokens.iter());
        parser.parse();

        dbg!(&parser.tree);

        // Create runtime
        let mut runtime = backend::eval::Runtime::initialize(parser.tree.into_iter());

        //dbg!(&parser);
    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
