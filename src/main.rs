#[path = "env/env.rs"] mod env;
#[path = "core/core.rs"] mod core;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "env/config.rs"] mod config;
#[path = "shell/cmd_shell/shell.rs"] mod shell;
#[path = "net/listener.rs"] mod net;
#[cfg(test)]
#[path = "../test/tests.rs"]
mod tests;

#[warn(non_camel_case_types)]
mod constants;
mod macros;

pub use constants::*;
pub use macros::*;
use std::io::{Error, Read, stdin, stdout, Write};
use std::thread;
use crate::core::{LDBValue};
pub use log::*;

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
    }
}

fn main() -> Result<(), Error> {
    log::set_logger(&MAIN_LOGGER);
    log::set_max_level(log::LevelFilter::Info);
    match launcher::launch() {
        Ok(_) => {},
        Err(_) => println!("Unknown error.")
    }
    return Ok(());
}
