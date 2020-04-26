use log::error;
use reqwest::Error;

pub trait Forecast {
    fn current(&self) -> Result<(), Error>;
    fn day(&self, days_num: Option<usize>) -> Result<(), Error>;

    fn log_response_error(&self, response: reqwest::blocking::Response) {
        error!(
            "Error while querying {}, response status {}",
            response.url().to_string(),
            response.status()
        );
        match response.text() {
            Err(err) => error!("Unable to parse response text: {}", err),
            Ok(response_text) => error!("Response content is: {}", response_text),
        }
    }
}
