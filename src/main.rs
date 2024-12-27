use std::{env, process};
use mingrep::Config;
use clap :: {Arg, Command};

fn main() {
    let matches = Command::new("minigrep")
        .version("1.1")
        .author("Rudransh Singh Tomar <trudranshsingh2003@gmail.com>")
        .about("Search for a string in a file")
        .arg(
            Arg::new("query")
                .help("The string to search for")
                .required(true),
            
        )
        .arg(
            Arg::new("file_path")
                .help("The file to search in")
                .required(true)
        )
        .arg(
            Arg::new("ignore_case")
                .long("ignore-case")
                .help("Perform a case-insensitive search"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("File")
                .help("Write search results to a file"),
        )
        .get_matches();

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


