#[macro_use]
extern crate serde;
extern crate log;
extern crate reqwest;
extern crate serde_derive;
extern crate lazy_static;
extern crate chrono;
extern crate clap;


use fc::base::Forecast;
use log::{debug, LevelFilter};
use reqwest::Error;
use std::str::FromStr;
use clap::{App, Arg};

mod configure;
mod fc;
mod logger;
mod utils;

fn parse_args() -> clap::ArgMatches {
    return App::new("Weather Forecast")
    .version("0.1.1")
    .author("Osadchuk Aleksey <aleosd@gmail.com>")
    .about("Prints weather forecast to console")
    .arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true),
    )
    .arg(
        Arg::new("log-level")
            .short('l')
            .long("log-level")
            .takes_value(true)
            .possible_values(&["info", "debug", "warn", "error"])
            .about("Set log level"),
    )
    .arg(
        Arg::new("day")
            .short('d')
            .long("day")
            .conflicts_with("week")
            .about("Show daily forecast"),
    )
    .arg(
        Arg::new("week")
            .short('w')
            .long("week")
            .conflicts_with("day")
            .about("Show weekly forecast"),
    )
    .arg(
        Arg::new("city")
            .long("city")
            .takes_value(true)
            .about("Change default city_id"),
    )
    .get_matches()
}

fn main() -> Result<(), Error> {
    let mut config = configure::load_config(None).unwrap();
    let args = parse_args();

    // setup logging
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

    // update config from args
    if args.is_present("city") {
        let city_id: u32 = args.value_of_t_or_exit("city");
        debug!("Got {} as a city_id from command line", city_id);
        config.city_id = args.value_of("city").unwrap().parse().unwrap();
    }

    // get forecast
    let forecaster = fc::openweathermap::OpenWeatherForecastRunner {
        config: config,
    };

    if args.is_present("day") {
        forecaster.day(Some(1))?;
    } else if args.is_present("week") {
        forecaster.day(Some(7))?;
    } else {
        forecaster.current()?;
    }
    Ok(())
}
