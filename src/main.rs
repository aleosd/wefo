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

mod configure;
mod fc;
mod logger;
mod utils;

fn main() -> Result<(), Error> {
    let config = configure::load_config(None).unwrap();
    log::set_logger(&logger::LOGGER).unwrap();
    log::set_max_level(
        LevelFilter::from_str(&config.system_config.log_level).unwrap_or(LevelFilter::Warn),
    );
    let forecaster = fc::openweathermap::OpenWeatherForecastRunner {
        config: config,
    };
    forecaster.current()?;
    Ok(())
}
