use super::super::configure;
use super::super::utils;
use super::base;
use log::{debug, info, error};
use chrono::{TimeZone, Local};
use std::convert::TryInto;

use lazy_static::lazy_static;
use reqwest::Error;
use std::collections::HashMap;

const DEGREE_SYMBOL: char = '\u{00B0}';

lazy_static! {
    static ref ICON_TO_SYMBOL: HashMap<String, char> = {
        let mut map = HashMap::new();
        map.insert("01d".to_owned(), '\u{263C}');
        map.insert("01n".to_owned(), '\u{263C}');
        map.insert("02d".to_owned(), '\u{1F324}');
        map.insert("02n".to_owned(), '\u{1F324}');
        map.insert("03d".to_owned(), '\u{1F325}');
        map.insert("03n".to_owned(), '\u{1F325}');
        map.insert("04d".to_owned(), '\u{2601}');
        map.insert("04n".to_owned(), '\u{2601}');
        map.insert("09d".to_owned(), '\u{1F327}');
        map.insert("09n".to_owned(), '\u{1F327}');
        map.insert("10d".to_owned(), '\u{1F326}');
        map.insert("10n".to_owned(), '\u{1F326}');
        map.insert("11d".to_owned(), '\u{1F329}');
        map.insert("11n".to_owned(), '\u{1F329}');
        map.insert("13d".to_owned(), '\u{2603}');
        map.insert("13n".to_owned(), '\u{2603}');
        map.insert("50d".to_owned(), '\u{1F32B}');
        map.insert("50n".to_owned(), '\u{1F32B}');
        map
    };
}

#[derive(Deserialize, Debug)]
struct Days5Forecast {
    cnt: i8,
    list: Vec<OpenWeatherForecast>
}


#[derive(Deserialize, Debug)]
struct OpenWeatherForecast {
    dt: u32,
    main: Main,
    weather: Vec<WeatherCondition>,
    wind: Wind,
    dt_txt: String,
}


#[derive(Deserialize, Debug)]
struct OpenWeatherCurrent {
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
            let forecast_data: OpenWeatherCurrent = response.json()?;
            println!(
                "Collected at {}\n{}  {}\nTemperature: {}{}C, Feels like: {}{}C\nWind: {}m/s",
                chrono::Utc.timestamp(forecast_data.dt.try_into().unwrap(), 0).with_timezone(&Local),
                ICON_TO_SYMBOL.get(&forecast_data.weather[0].icon).unwrap_or(&'\u{0020}'),
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

    fn day5(&self) -> Result<(), Error> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?id={city_id}&appid={api_key}&units=metric",
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
            let forecast_data: Days5Forecast = response.json()?;
            for item in forecast_data.list.iter() {
                println!(
                    "{}\n{}  {}\nTemperature: {}{}C, Feels like: {}{}C\nWind: {}m/s\n",
                    chrono::Utc.timestamp(item.dt.try_into().unwrap(), 0).with_timezone(&Local),
                    ICON_TO_SYMBOL.get(&item.weather[0].icon).unwrap_or(&'\u{0020}'),
                    utils::uppercase_first_letter(&item.weather[0].description),
                    item.main.temp,
                    DEGREE_SYMBOL,
                    item.main.feels_like,
                    DEGREE_SYMBOL,
                    item.wind.speed,
                );
            }
        }
        Ok(())
    }
}