use std::{env, process};
use mingrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("String that we have to find {}", config.query);
    if let Err(e) = mingrep::run(config){
        println!("Application Error {e}");
        process::exit(1)
    };    
}

