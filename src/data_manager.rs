use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom, Read};
use crate::error;
use crate::DataError;
use std::os::windows::fs::FileExt;

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

pub fn create_database(path: &str, name: &str) -> Result<(), DataError> {
    let mut file = File::create(path).unwrap();

    let header = DATABASE_NAME.to_owned() + ":" + VERSION + ":";
    let metadata = header + name;
    let meta_len = metadata.len();

    let mut bin: Vec<u8> = Vec::new();
    bin.push(meta_len as u8);
    bin.append(&mut encode(metadata));

    match file.write(bin.as_slice()) {
        Ok(n) => {}
        Err(n) => {
            error!("Error when creating database.");
        }
    }

    Ok(())
}

pub fn create_table(path: &str, name: &str, types: &str) {
    let mut file = OpenOptions::new().write(true).append(true).open(path).unwrap();
    let mut rfile = File::open(path).unwrap();

    let metadata = name.to_string() + types;
    let meta_len = metadata.len();

    let mut bin: Vec<u8> = Vec::new();
    bin.push(meta_len as u8);
    bin.append(&mut encode(metadata));

    let size = rfile.read_to_end(&mut Vec::new()).unwrap() as u64;
    match file.seek_write(bin.as_slice(), size) {
        Ok(n) => {}
        Err(n) => {
            error!("Error when creating table.");
        }
    }
}