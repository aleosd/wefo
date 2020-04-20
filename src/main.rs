#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
extern crate log;

use log::{debug, error, LevelFilter};
use std::str::FromStr;
use reqwest::Error;

mod configure;
mod logger;

#[derive(Deserialize, Debug)]
struct WeatherForecast {
    name: String,
    visibility: u32,
    dt: u32,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: u32,
    humidity: u32,
}

fn main() -> Result<(), Error> {
    let config = configure::load_config(None).unwrap();
    log::set_logger(&logger::LOGGER).unwrap();
    log::set_max_level(LevelFilter::from_str(&config.system_config.log_level).unwrap_or(LevelFilter::Warn));
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}",
        city_id = config.city_id,
        api_key = config.openweathermap_api_key
    );
    debug!("Requesting {}", url);
    let response = reqwest::blocking::get(&url)?;
    if response.status() != 200 {
        error!("Error while querying {}, response status {}", url, response.status());
        match response.text() {
            Err(err) => error!("Unable to parse response text: {}", err),
            Ok(response_text) => error!("Response content is: {}", response_text)
        }
    } else {
        debug!("Got results from {}", url);
        let forecast_data: WeatherForecast = response.json()?;
        println!("{:?}", forecast_data);
    }
    Ok(())
}
