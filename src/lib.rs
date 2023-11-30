use std::{io::Error, env};

mod file_handler;

pub struct EnvironmentVariables{
    pub case_insensitive: bool
}

pub struct Config {
    pub search_string: String,
    pub file: String,
    pub env_variables: EnvironmentVariables
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str>{
        if args.len() != 3 {
            return Err("Incorrect number of arguments. Expected 2 arguments.\nShould send string to search and file location in this order.")
            //panic!("Incorrect number of arguments. Expected 2 arguments.\nShould send string to search and file location in this order.");
        }
        Ok(Config { search_string: args[1].to_string(), file: args[2].to_string(), env_variables: EnvironmentVariables { case_insensitive: env::var("LOWER_CASE_SEARCH").is_ok()}})
    }
}

pub fn read_file_content(file_path: &str) -> Result<String, Error> {
    let content = file_handler::read_file_content(file_path)?;
    println!("File content: {}", content);
    
    Ok(content)
}

pub fn search_on_content<'a>(search: &'a str, file_content: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();
    for line in file_content.lines(){
        if line.contains(search){
            result.push(line);
        }
    }
    result
}

pub fn search_on_content_case_insensitive<'a>(search: &'a str, file_content: &'a str) -> Vec<&'a str>{
    let mut result: Vec<&str> = Vec::new();
    for line in file_content.lines(){
        if line.to_lowercase().contains(search.to_lowercase().as_str()){
            result.push(line);
        }
    }
    result
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_on_content(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_on_content_case_insensitive(query, contents)
        );
    }
}