use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::os::windows::fs::FileExt;
use crate::error::DataError;
use crate::error;
use std::io::SeekFrom;
use std::borrow::Borrow;

const DATABASE_NAME: &str = "littledb";
const VERSION: &str = "v0.1";

pub fn decode(bin: Vec<u8>) -> String {
    let mut res: Vec<u8> = Vec::new();
    for i in bin { res.push(i >> 1) }

    String::from_utf8(res).unwrap()
}

pub fn encode(text: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in text.into_bytes() { res.push(i << 1) }

    res
}

pub fn load_database_metadata() {

}

pub unsafe fn init_database() {
    let filename = format!("{}/{}", crate::FOLDER_PATH.with(|a| a.take()), "data.db");
    let mut file = File::open(filename.clone());
    let header = DATABASE_NAME.to_owned() + ":" + VERSION + ":";
    let metadata = header;

    write_as_binary(&*filename, metadata, "database initialization error");
}


pub fn append_as_binary(path: &str, text: String, error_text: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(path).unwrap();

    let meta_len = text.len();

    let mut bin: Vec<u8> = Vec::new();
    bin.push(meta_len as u8);
    bin.append(&mut encode(text));

    let size = file.read_to_end(&mut Vec::new()).unwrap() as u64;
    match file.seek_write(bin.as_slice(), size) {
        Ok(n) => {}
        Err(n) => {
            error!("{}", error_text);
        }
    }
}

pub fn write_as_binary(filename: &str, text: String, error_text: &str) {
    let mut file = File::create(filename).unwrap();
    let meta_len = text.len() as u8;

    let mut bin: Vec<u8> = Vec::new();
    bin.push(meta_len as u8);
    bin.append(&mut encode(text));

    match file.write(bin.as_slice()) {
        Ok(n) => {}
        Err(n) => {
            error!("{}", error_text);
        }
    }
}