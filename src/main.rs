use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dbga = cfg!(debug_assertions);

    if args.len() == 2 && dbga {
        // File path is specified
    } else if args.len() < 2 && dbga {
        eprintln!("Please specify a file path");
        std::process::exit(1);
    } else {
        todo!("Non DBGA logic");
    }
}
