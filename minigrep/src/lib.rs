use std::error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err(
                "Should have two arguments, one for the query and another for the file path.",
            );
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.file_path)?;

    println!("Content:\n{content}");

    Ok(())
}
