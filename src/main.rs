use mahgrep::{read_file_content, search_on_content, search_on_content_case_insensitive, Config};
use std::env::args;
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(conf).unwrap_or_else(|err| {
        println!("Problem running the program: {err}");
        process::exit(2);
    });
}

fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let file_content = read_file_content(&conf.file)?;
    let returned_value;
    if conf.env_variables.case_insensitive {
        returned_value =
            search_on_content_case_insensitive(&conf.search_string, file_content.as_str());
    } else {
        returned_value = search_on_content(&conf.search_string, file_content.as_str());
    }
    println!("Strings found: ");
    for line in returned_value {
        println!("{}", line);
    }
    Ok(())
}
