use reqwest::Error;

pub trait Forecast {
    fn current(&self) -> Result<(), Error>;
    fn day5(&self) -> Result<(), Error>;
}
