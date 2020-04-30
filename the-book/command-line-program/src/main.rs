use std::env;
use std::process;

use command_line_program as clp;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Use idioms
    let config = clp::Config::new(&args).unwrap_or_else(|err| {
        // Print error to stderr
        eprintln!("Cannot parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = clp::run(config) {
        eprintln!("Run error: {}", e);
        process::exit(1);
    }
}
