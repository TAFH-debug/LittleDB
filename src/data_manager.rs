use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom, Read};
use crate::error;
use crate::DataError;
use std::os::windows::fs::FileExt;
use crate::core::{write_as_binary, append_as_binary};

const DATABASE_NAME: &str = "littledb";
const VERSION: &str = "v0.1";

pub fn create_database(path: &str, name: &str) -> Result<(), DataError> {
    let header = DATABASE_NAME.to_owned() + ":" + VERSION + ":";
    let metadata = header + name;

    write_as_binary(path, metadata, "error when creating database");
    Ok(())
}

pub fn create_table(path: &str, name: &str, types: &str) {
    let metadata = name.to_string() + types;
    append_as_binary(path, metadata, "error when creating database");
}