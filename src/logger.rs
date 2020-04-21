extern crate chrono;

use log::{Level, Metadata, Record};

pub static LOGGER: Logger = Logger;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !record.target().starts_with("wefo") {
            return;
        }
        if self.enabled(record.metadata()) {
            println!(
                "{} - {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                record.args()
            );
        }
    }
    fn flush(&self) {}
}
