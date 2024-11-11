use std::{env, fmt::format, fs};

use frontend::parser;

mod errors;
mod frontend;
mod interep;

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

        let mut interep = interep::ir_gen::Interep::new(&parser.tree);
        interep.generate();

        dbg!(&interep);

        //dbg!(&parser);
    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
