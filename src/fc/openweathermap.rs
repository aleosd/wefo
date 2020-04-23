use super::super::configure;
use super::super::utils;
use super::base;
use log::{debug, error};

extern crate log;
extern crate reqwest;
use reqwest::Error;

const DEGREE_SYMBOL: char = '\u{00B0}';

#[derive(Deserialize, Debug)]
struct OpenWeatherForecast {
    name: String,
    visibility: u32,
    dt: u32,
    main: Main,
    weather: Vec<WeatherCondition>,
    wind: Wind,
    sys: Sys,
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

#[derive(Deserialize, Debug)]
struct WeatherCondition {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: u16,
}

#[derive(Deserialize, Debug)]
struct Sys {
    sunrise: u32,
    sunset: u32,
}

pub struct OpenWeatherForecastRunner {
    pub config: configure::WeFoConfig,
}

impl base::Forecast for OpenWeatherForecastRunner {
    fn current(&self) -> Result<(), Error> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}&units=metric",
            city_id = self.config.city_id,
            api_key = self.config.openweathermap_api_key
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
            println!(
                "{}\nTemperature: {}{}C, Feels like: {}{}C\nWind: {}m/s",
                utils::uppercase_first_letter(&forecast_data.weather[0].description),
                forecast_data.main.temp,
                DEGREE_SYMBOL,
                forecast_data.main.feels_like,
                DEGREE_SYMBOL,
                forecast_data.wind.speed,
            );
        }
        Ok(())
    }
}
