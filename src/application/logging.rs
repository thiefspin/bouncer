extern crate chrono;
extern crate fern;

use std::env;
use std::str::FromStr;
use fern::Dispatch;

use log::LevelFilter;

pub struct Logging;

impl Logging {
    pub(crate) fn init() {
        let logger = Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(get_log_level())
            .chain(std::io::stdout());
        log_to_file(logger)
            .apply()
            .expect("Could not initialize logging")
    }
}

fn log_to_file(logger: Dispatch) -> Dispatch {
    match env::var("LOGGING_FILE_NAME") {
        Ok(file_name) => {
            logger.chain(fern::log_file(file_name).unwrap())
        }
        Err(_) => logger
    }
}

fn get_log_level() -> LevelFilter {
    let error = "Could not parse the given `LOGGING_LEVEL`. Hint: [Off, Error, Warn, Info, Debug, Trace]";
    match env::var("LOGGING_LEVEL") {
        Ok(level) => {
            LevelFilter::from_str(level.as_str()).expect(error)
        }
        Err(_) => LevelFilter::Debug
    }
}