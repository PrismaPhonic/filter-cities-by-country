use std::env;
use std::process;

use filter_cities_by_country::Config;
use filter_cities_by_country;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = filter_cities_by_country::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
