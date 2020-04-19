#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
use reqwest::Error;

mod configure;

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
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}",
        city_id = config.city_id,
        api_key = config.openweathermap_api_key
    );
    let response = reqwest::blocking::get(&url)?;
    if response.status() != 200 {
        println!("Called url: {}", url);
        println!("{:?}", response);
    }

    let users: WeatherForecast = response.json()?;
    println!("{:?}", users);
    Ok(())
}
