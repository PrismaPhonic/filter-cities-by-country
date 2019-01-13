use std::fs::File;
use std::{env, fs};
use std::error::Error;
use std::io::{Write, BufReader, BufRead};
use regex::Regex;

pub struct Config {
    pub input: String,
    pub output: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file to analyze!"),
        };

        let output = match args.next() {
            Some(arg) => arg,
            None => return Err("You need to provide the name of the file I will write to."),
        };

        Ok(Config { input, output })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.input)?;

    let mut output = File::create(&config.output)?;
    
    // // These regex match lines for canada or united states
    // // for reference
    // let usa = Regex::new(r"PPL\s+US");
    // let canada = Regex::new(r"PPL\s+CA");

    // matches either canada or united states in one regex
    let either = Regex::new(r"PPL\s+(US|CA)")?;

    // matches city-name from line
    let city_re = Regex::new(r"\d+\s(\w+)")?;
    for line in BufReader::new(file).lines() {
        let unwrapped = line?;
        if either.is_match(&unwrapped) {
            let city = city_re.captures_iter(&unwrapped).next().unwrap();
            write!(output, "{}\n", city.get(1).unwrap().as_str())?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
