use super::super::configure;
use super::super::utils;
use super::base;
use chrono::{Local, TimeZone};
use log::debug;
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
    list: Vec<OpenWeatherForecast>,
}

#[derive(Deserialize, Debug)]
struct OpenWeatherForecast {
    dt: u32,
    main: Main,
    weather: Vec<WeatherCondition>,
    wind: Wind,
    dt_txt: String,
}

impl OpenWeatherForecast {
    pub fn fc_date(&self) -> String {
        return utils::format_date(
            chrono::Utc
                .timestamp(self.dt.try_into().unwrap(), 0)
                .with_timezone(&Local),
        );
    }
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

impl OpenWeatherCurrent {
    fn updated_at(&self) -> String {
        return utils::format_date(
            chrono::Utc
                .timestamp(self.dt.try_into().unwrap(), 0)
                .with_timezone(&Local),
        );
    }

    fn sunrise(&self) -> String {
        return utils::format_date(
            chrono::Utc
                .timestamp(self.sys.sunrise.try_into().unwrap(), 0)
                .with_timezone(&Local),
        );
    }

    fn sunset(&self) -> String {
        return utils::format_date(
            chrono::Utc
                .timestamp(self.sys.sunset.try_into().unwrap(), 0)
                .with_timezone(&Local),
        );
    }
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

impl OpenWeatherForecastRunner {
    fn current_weather_url(&self) -> String {
        return format!(
            "https://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}&units=metric",
            city_id = self.config.city_id,
            api_key = self.config.openweathermap_api_key
        );
    }

    fn weather_forecast_url(&self) -> String {
        return format!(
            "https://api.openweathermap.org/data/2.5/forecast?id={city_id}&appid={api_key}&units=metric",
            city_id = self.config.city_id,
            api_key = self.config.openweathermap_api_key
        );
    }

    fn print_forecast_item(&self, forecast_data: &OpenWeatherForecast) {
        println!(
            "{}\n{}  {}\nTemperature: {}{}C, feels like: {}{}C\nWind: {}m/s\n",
            forecast_data.fc_date(),
            ICON_TO_SYMBOL
                .get(&forecast_data.weather[0].icon)
                .unwrap_or(&'\u{0020}'),
            utils::uppercase_first_letter(&forecast_data.weather[0].description),
            forecast_data.main.temp,
            DEGREE_SYMBOL,
            forecast_data.main.feels_like,
            DEGREE_SYMBOL,
            forecast_data.wind.speed,
        );
    }
}

impl base::Forecast for OpenWeatherForecastRunner {
    fn current(&self) -> Result<(), Error> {
        let url = self.current_weather_url();
        debug!("Requesting {}", url);
        let response = reqwest::blocking::get(&url)?;
        if response.status() != 200 {
            self.log_response_error(response);
        } else {
            debug!("Got results from {}", url);
            let forecast_data: OpenWeatherCurrent = response.json()?;
            println!(
                "Updated at {}\n{}  {}\nTemperature: {}{}C, feels like: {}{}C\nWind: {}m/s\nSunrise: {}\nSunset: {}",
                forecast_data.updated_at(),
                ICON_TO_SYMBOL.get(&forecast_data.weather[0].icon).unwrap_or(&'\u{0020}'),
                utils::uppercase_first_letter(&forecast_data.weather[0].description),
                forecast_data.main.temp,
                DEGREE_SYMBOL,
                forecast_data.main.feels_like,
                DEGREE_SYMBOL,
                forecast_data.wind.speed,
                forecast_data.sunrise(),
                forecast_data.sunset()
            );
        }
        Ok(())
    }

    fn day(&self, days_num: Option<usize>) -> Result<(), Error> {
        debug!("Collecting forecat...");
        let url = self.weather_forecast_url();
        debug!("Requesting {}", url);
        let response = reqwest::blocking::get(&url)?;
        if response.status() != 200 {
            self.log_response_error(response);
        } else {
            debug!("Got results from {}", url);
            let forecast_data: Days5Forecast = response.json()?;
            if days_num.is_some() && days_num.unwrap() * 4 < forecast_data.list.len() {
                for index in 0..days_num.unwrap() * 4 {
                    self.print_forecast_item(&forecast_data.list[index]);
                }
            } else {
                for item in forecast_data.list.iter() {
                    self.print_forecast_item(item);
                }
            }
        }
        Ok(())
    }
}
