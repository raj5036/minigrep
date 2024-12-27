use std::fs;
use std::error::Error;

pub fn run (config: Config)->Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>)-> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        // Config { query: query, file_path: filename }
        Ok(Config { query, file_path: filename })
    }
}