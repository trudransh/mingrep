use std::{env, process};
use mingrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // standard library provides the eprintln! macro that prints to the standard error stream,
        process::exit(1)
    });
    println!("String that we have to find is : {}", config.query);
    println!("File in which we are searching is : {}", config.file_path);
    if let Err(e) = mingrep::run(config){
        eprintln!("Application Error {e}"); // * WE are using eprintlin!() macro to print error to standarda error stream not to print it on output.txt which is for standard output
        process::exit(1)
    };    
}


