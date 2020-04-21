#[macro_use]
extern crate serde;
extern crate log;
extern crate reqwest;
extern crate serde_derive;

use fc::base::Forecast;
use log::LevelFilter;
use reqwest::Error;
use std::str::FromStr;

mod configure;
mod fc;
mod logger;

fn main() -> Result<(), Error> {
    let config = configure::load_config(None).unwrap();
    log::set_logger(&logger::LOGGER).unwrap();
    log::set_max_level(
        LevelFilter::from_str(&config.system_config.log_level).unwrap_or(LevelFilter::Warn),
    );
    let forecaster = fc::openweathermap::OpenWeatherForecastRunner {};
    forecaster.current(config)?;
    Ok(())
}
