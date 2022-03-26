use std::borrow::Borrow;
use std::fmt::Debug;
use std::path::Path;

extern crate clap;
use clap::Parser;
use crate::constants::Mode;

///SQL-like simple database management system written on rust.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Mode of application start. "web" or "local".
    #[clap(short, long, default_value = "local")]
    mode: String,
    ///Path to folder where locate database. Absolute or relative.
    #[clap(short, long, default_value = "")]
    folder: String,
}

///Launch DBMS
pub fn launch() -> Result<(), ()> {
    crate::env::config::load();

    let args = Args::parse();

    match &*args.mode.clone() {
        "local" => {
            let mut mode = crate::constants::MODE.lock().unwrap();
            *mode = Mode::LOCAL;
        },
        "web" => {
            let mut mode = crate::constants::MODE.lock().unwrap();
            *mode = Mode::WEB;
        },
        _ => {
            println!("Unknown mode!");
            return Ok(());
        }
    }

    if args.folder.clone() == "" {
        println!("Error: Missing required command-line option --folder.");
        println!("For more information try --help.");
        return Ok(());
    }
    if !Path::new(&args.folder).exists() {
        println!("Error: This path is not exists.");
        return Ok(());
    }

    crate::constants::FOLDER_PATH.with(|g| {
        g.replace(args.folder.clone());
    });

    crate::shell::shell::start_shell();
    //TODO port
    Ok(())
}
