use reqwest::Error;

pub trait Forecast {
    fn current(&self) -> Result<(), Error>;
}
