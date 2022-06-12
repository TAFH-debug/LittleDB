pub mod core;
pub mod env;
pub mod net;
pub mod shell;
pub mod constants;
pub mod macros;
pub mod parser;

use log::*;


extern crate log;

struct MainLogger;
static MAIN_LOGGER: MainLogger = MainLogger;

impl Log for MainLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        //does nothing
    }
}

fn main() -> Result<(), ()> {
    set_logger(&MAIN_LOGGER).expect("TODO: panic message");
    set_max_level(LevelFilter::Info);
    shell::launcher::launch().unwrap_or_else(|_| println!("Unknown error!"));
    return Ok(());
}
