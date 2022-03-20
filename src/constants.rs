use std::{
    cell::RefCell
};

pub enum Mode {
    NONE,
    WEB,
    LOCAL
}

pub enum StorageType {
    TABLE,
    SINGLE_OBJECTS
}

thread_local! {
    pub static FOLDER_PATH: RefCell<String> = RefCell::new(String::new());
}
/**
Returns database main file name.
*/
pub fn get_db_file() -> String {
    format!("{}/{}.{}",
            FOLDER_PATH.with(|a| a.clone().take()),
            FILE_NAME,
            MAIN_FILE_EXTENSION)
}

/**
Returns table data file name.
*/
pub fn get_tbl_file() -> String {
    format!("{}/{}.{}",
        FOLDER_PATH.with(|a| a.clone().take()),
        FILE_NAME,
        STORAGE_DATA_EXTENSION
    )
}
pub const FILE_NAME: &str = "data";

pub static mut MODE: Mode = Mode::NONE;
pub static CONFIG_FILE: &str = "config.json";
pub static MAIN_FILE_EXTENSION: &str = "db";
pub static STORAGE_DATA_EXTENSION: &str = "stg_data";