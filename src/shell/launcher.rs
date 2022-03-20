use std::fmt::Debug;
use std::path::Path;

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
}

/**
Launch DBMS.
*/
pub fn launch() -> Result<(), String> {
    crate::config::load();

    let args = Args::parse();

    match &*args.mode.clone() {
        "local" => unsafe {
            crate::MODE = crate::Mode::LOCAL;
        },
        "web" => unsafe {
            crate::MODE = crate::Mode::WEB;
        }
        _ => {
            println!("Unknown mode!");
            std::process::exit(0);
        }
    }

    if args.folder.clone() == "" {
        println!("Error: Missing required command-line option --folder.");
        println!("For more information try --help.");
        std::process::exit(0);
    }
    if !Path::new(&args.folder).exists() {
        println!("Error: This path is not exists.");
        std::process::exit(0);
    }

    crate::FOLDER_PATH.with(|g| {
        g.replace(args.folder.clone());
    });

    crate::shell::start_shell();
    //TODO port
    Ok(())
}