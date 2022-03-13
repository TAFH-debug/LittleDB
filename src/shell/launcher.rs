use std::error::Error;
use std::fmt::{Display, Debug, Formatter};

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
    folder: String,
    ///Name of database.
    #[clap(short, long, default_value = "")]
    name: String
}

/**
Launch DBMS.
*/
pub fn launch() -> Result<(), String> {
    crate::config::load();
    //crate::shell::start_shell();

    let a = Args::parse();

    match &*a.mode.clone() {
        "local" => unsafe {
            crate::MODE = crate::Mode::LOCAL;
        },
        "web" => unsafe {
            crate::MODE = crate::Mode::WEB;
        }
        _ => {
            panic!("Unknown mode.");
        }
    }

    unsafe { crate::FOLDER_PATH.with(|g| {
        g.replace(a.folder.clone());
    }); }
    unsafe { crate::DB_NAME.with(|g| {
        g.replace(a.name.clone());
    })}

    //TODO port
    Ok(())
}