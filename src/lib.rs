use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{env, fs};

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
    let city_lat_long_country_re =
        Regex::new(r"^\d+\s(.+[-]?\d+\.\d+\s[-]?\d+\.\d+\s+P\s+PPL\s\D+)\d+")?;
    for line in BufReader::new(file).lines() {
        let unwrapped = line?;
        if either.is_match(&unwrapped) {
            let capture = city_lat_long_country_re
                .captures_iter(&unwrapped)
                .next()
                .unwrap();
            let cropped = capture.get(1).unwrap().as_str();

            let filter_more_re = Regex::new(r"^([\w‘'-\.]+\s[\w‘'-\.]+)\D*([-]?\d+\.\d+\s[-]?\d+\.\d+\s+P\s+PPL\s\D+)")?;
            let capture_2 = filter_more_re.captures_iter(&cropped).next().unwrap();
            let city = capture_2.get(1).unwrap().as_str();
            let rest = capture_2.get(2).unwrap().as_str();
            write!(output, "{} {}\n", city, rest)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
