/// Implementation of cp
/// e.g. `copy <sourcefile> <destination>`
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: {} <source_file> <destination_file>", args[0]);
        process::exit(1); 
    }

    let source = &args[1];
    let destination = &args[2];

    match fs::copy(source, destination) {
        Ok(..) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
