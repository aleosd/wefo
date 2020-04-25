#[macro_use]
extern crate serde;
extern crate log;
extern crate reqwest;
extern crate serde_derive;
extern crate lazy_static;
extern crate chrono;


use fc::base::Forecast;
use log::LevelFilter;
use reqwest::Error;
use std::str::FromStr;
use clap::{App, Arg};

mod configure;
mod fc;
mod logger;
mod utils;

fn parse_args() -> clap::ArgMatches {
    return App::new("MyApp")
    .version("0.1.0")
    .author("Osadchuk Aleksey <aleosd@gmail.com>")
    .about("Prints weather forecast to console")
    .arg(
        Arg::with_name("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("log-level")
            .short('l')
            .long("log-level")
            .takes_value(true)
            .possible_values(&["info", "debug", "warn", "error"])
            .help("Set log level"),
    )
    .arg(
        Arg::with_name("day5")
            .long("day5")
            .conflicts_with("daily")
            .help("Show hourly forecast"),
    )
    .arg(
        Arg::with_name("daily")
            .short('d')
            .long("daily")
            .conflicts_with("day5")
            .help("Show daily forecast"),
    )
    .get_matches()
}

fn main() -> Result<(), Error> {
    let config = configure::load_config(None).unwrap();
    let args = parse_args();

    // setup config
    log::set_logger(&logger::LOGGER).unwrap();
    let log_level: String;
    if args.is_present("log-level") {
        log_level = args.value_of("log-level").unwrap().to_string();
    } else {
        log_level = config.system_config.log_level.to_string();
    }
    log::set_max_level(
        LevelFilter::from_str(&log_level).unwrap_or(LevelFilter::Warn),
    );

    // get forecast
    let forecaster = fc::openweathermap::OpenWeatherForecastRunner {
        config: config,
    };

    if args.is_present("day5") {
        forecaster.day5()?;
    } else {
        forecaster.current()?;
    }
    Ok(())
}
