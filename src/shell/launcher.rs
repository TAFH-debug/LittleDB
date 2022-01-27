use std::error::Error;
use std::fmt::{Display, Debug, Formatter};
use crate::error::*;
use crate::error;

extern crate clap;
use clap::Parser;

///SQL-like simple database management system written on rust.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Mode of application start. "web" or "local".
    #[clap(short, long, default_value = "local")]
    mode: String,
    ///Port that listens to connections.
    #[clap(short, long, default_value = "5000")]
    port: u32,
    ///Path to folder where locate database. Absolute or relative.
    #[clap(short, long, default_value = "")]
    folder: String
}

/**
Launch DBMS.
*/
pub fn launch() -> Result<(), DataError> {
    crate::config::load();
    crate::shell::launch_shell();
    let a = Args::parse();

    match &*a.mode {
        "local" => unsafe {
            crate::MODE = crate::Mode::LOCAL;
        },
        "web" => unsafe {
            crate::MODE = crate::Mode::WEB;
        }
        _ => {
            error!("Unknown mode.");
        }
    }
    //TODO port
    Ok(())
}