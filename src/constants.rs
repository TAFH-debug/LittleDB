use std::cell::RefCell;
use std::sync::Mutex;
use clap::lazy_static::lazy_static;

pub enum Mode {
    NONE,
    WEB,
    LOCAL,
}

thread_local! {
    pub static FOLDER_PATH: RefCell<String> = RefCell::new(String::new());
}
///Returns database main file name.
pub fn get_db_file() -> String {
    format!(
        "{}/{}.{}",
        FOLDER_PATH.with(|a| a.clone().take()),
        FILE_NAME,
        MAIN_FILE_EXTENSION
    )
}

///Returns table data file name.
pub fn get_tbl_file() -> String {
    format!(
        "{}/{}.{}",
        FOLDER_PATH.with(|a| a.clone().take()),
        FILE_NAME,
        STORAGE_DATA_EXTENSION
    )
}
pub const FILE_NAME: &str = "data";

lazy_static! {
    pub static ref PORT: Mutex<u32> = Mutex::new(5000);
    pub static ref MODE: Mutex<Mode> = Mutex::new(Mode::NONE);
}
pub static CONFIG_FILE: &str = "config.json";
pub static MAIN_FILE_EXTENSION: &str = "db";
pub static STORAGE_DATA_EXTENSION: &str = "stg_data";
