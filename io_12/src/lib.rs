use std::env;
// use std::error::Error;

// parse args
pub fn parse_args() -> Result<Config, &'static str> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    Ok(Config::new(&args))
}

#[derive(Debug)]
pub struct Config {
    content: String,
    search_string: String,
}

impl Config {
    pub fn new(s: &[String]) -> Config {
        println!("create a config");
        Config {
            content: s[1].clone(),
            search_string: s[2].clone(),
        }
    }

    pub fn print_config(&self) {
        println!("The searched content is : {}", self.content);
        println!("The searching string is : {}", self.search_string);
    }
}