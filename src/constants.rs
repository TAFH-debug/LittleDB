use std::cell::RefCell;

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

pub const FILE_NAME: &str = "data";
pub static CONFIG_FILE: &str = "config.json";
pub static MAIN_FILE_EXTENSION: &str = "db";
pub static STORAGE_DATA_EXTENSION: &str = "stg_data";
