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

    match fs::copy(&args[1], &args[2]) {
        Ok(..) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
