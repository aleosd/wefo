use super::super::configure;
use reqwest::Error;

pub trait Forecast {
    fn current(&self, config: configure::WeFoConfig) -> Result<(), Error>;
}
