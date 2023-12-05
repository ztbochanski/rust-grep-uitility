use std::env;
use std::process;

use rustgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|  {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rustgrep::run(config) {
        process::exit(1);
    }  
}
