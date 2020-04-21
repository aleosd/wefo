use super::super::configure;
use super::base;
use log::{debug, error};

extern crate reqwest;
extern crate log;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct OpenWeatherForecast {
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

pub struct OpenWeatherForecastRunner {}

impl base::Forecast for OpenWeatherForecastRunner {
    fn current(&self, config: configure::WeFoConfig) -> Result<(), Error> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}",
            city_id = config.city_id,
            api_key = config.openweathermap_api_key
        );
        debug!("Requesting {}", url);
        let response = reqwest::blocking::get(&url)?;
        if response.status() != 200 {
            error!(
                "Error while querying {}, response status {}",
                url,
                response.status()
            );
            match response.text() {
                Err(err) => error!("Unable to parse response text: {}", err),
                Ok(response_text) => error!("Response content is: {}", response_text),
            }
        } else {
            debug!("Got results from {}", url);
            let forecast_data: OpenWeatherForecast = response.json()?;
            println!("{:?}", forecast_data);
        }
        Ok(())
    }
}
