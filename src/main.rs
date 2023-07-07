use std::env;
use std::process;

use cracking_codes_with_rust::Config;
// use cracking_codes_with_rust::chapters::chapter_02::caesar;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Program parsing arguments: {err}");
        process::exit(1);
    });

    println!("mode: {}", config.mode);
    println!("file: {}", config.file_path);

    if let Err(e) = cracking_codes_with_rust::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


