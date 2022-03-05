use std::{
    fs::{ File, OpenOptions },
    io::{ prelude::*, SeekFrom},
    borrow::Borrow
};
use crate::{
    error,
    error::DataError,
    DBMS_NAME,
    VERSION
};

pub fn decode(bin: Vec<u8>) -> Result<String, String> {
    let mut res: Vec<u8> = Vec::new();
    for i in bin { res.push(i >> 1) }

    match String::from_utf8(res) {
        Ok(n) => Ok(n),
        Err(n) => Err(String::from("String decoding error"))
    }
}

pub fn encode(text: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in text.into_bytes() { res.push(i << 1) }
    res
}

pub fn load_database_metadata() {
    //TODO
}

pub unsafe fn init_database() {
    let filename = format!("{}/{}", crate::FOLDER_PATH.with(|a| a.take()), "data.db");
    let mut file = File::open(filename.clone());
    let header = DBMS_NAME.to_owned() + ":" + VERSION + ":";
    let metadata = header;

    write_as_binary(&*filename, metadata);
}

/**
Appends data text to end of file.
Example:
```
fn main() {
    append_as_binary("some.txt", "data".to_string());
}
```
 */
pub fn append_as_binary(path: &str, text: String) -> Result<(), String> {
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
    file.seek(SeekFrom::Start(size));

    match file.write(bin.as_slice()) {
        Ok(n) => Ok(()),
        Err(n) => Err(String::from("Error"))
    }
}

pub fn write_as_binary(filename: &str, text: String) -> Result<(), String> {
    let mut file = File::create(filename).unwrap();
    let meta_len = text.len() as u8;

    let mut bin: Vec<u8> = Vec::new();
    bin.push(meta_len as u8);
    bin.append(&mut encode(text));

    match file.write(bin.as_slice()) {
        Ok(n) => Ok(()),
        Err(n) => Err(String::from("Error"))
    }
}