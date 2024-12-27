
use std::error::Error;
use std::fs;
use std::env;


pub fn run (config : Config) -> Result<(), Box<dyn Error>> { // Boxing allows you to handle the errors Dynamically and will help you 
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub fn search <'a>(query : &str, contents : &'a str ) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();  // shadowing variable with the same name as params ALSO -> to_lowercase converted &str to String 
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){ // Here we cannot directly pass query cuz contains require (String slice not a string)
            results.push(line);
        }
    }
    results
}


pub struct Config{
    pub query : String,
    pub file_path : String,
    pub ignore_case : bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok(); // Here we doesn't care if the value of environment variable is true or false, If it is set then is_ok() will return true else false 

        Ok(Config { query, file_path , ignore_case})
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn case_sensitive (){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query,contents));
    }

    #[test]
    fn case_insensitive (){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
    );

    }
}