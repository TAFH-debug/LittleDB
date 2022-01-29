#[warn(non_camel_case_types)]

#[path = "module_interaction/interaction.rs"] mod inter;
#[path = "core/core_binary.rs"] mod bcore;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "other/error.rs"] mod error;
#[path = "env/config.rs"] mod config;
#[path = "shell/shell.rs"] mod shell;
#[path = "../test/io_test.rs"] mod test;
use std::fs::File;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error, Read};
use crate::inter::{ModuleRequest, Parameter};
use crate::bcore::BCRequestType;
use std::cell::RefCell;

pub enum Mode {
    NONE,
    WEB,
    LOCAL
}
thread_local! {
    pub static FOLDER_PATH: RefCell<String> = RefCell::new(String::new());
    pub static DB_NAME: RefCell<String> = RefCell::new(String::new());
}

pub static mut MODE: Mode = Mode::NONE;
pub static CONFIG_FILE: &str = "config.json";
pub static MAIN_FILE_EXTENSION: &str = "db";
pub static TABLE_DATA_EXTENSION: &str = "table_data";

fn main() -> Result<(), Error> {
    launcher::launch();
    unsafe {
        bcore::create_table("users".to_string(), ":int:string".to_string());
    }

    {
        test::main();
    }
    return Ok(());
}
