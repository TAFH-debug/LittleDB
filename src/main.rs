pub mod core;
pub mod env;
pub mod net;
pub mod shell;
pub mod constants;
pub mod macros;
pub mod parser;

use std::io::stdin;
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
        //TODO
    }
}

fn main() -> Result<(), ()> {
    let mut buf = String::new();
    stdin().read_line(&mut buf);
    //println!("{:#?}", parse_expr(buf));
    Ok(())
    /*
    set_logger(&MAIN_LOGGER);
    set_max_level(LevelFilter::Info);
    shell::launcher::launch().unwrap_or_else(|_| println!("Unknown error!"));
    return Ok(());
    */
}
