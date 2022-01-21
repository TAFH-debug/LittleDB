use std::fs::File;
use std::io::Write;
use crate::error;
use crate::DataError;

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

pub fn create_database(mut file: File, name: &str) -> Result<(), DataError> {
    let header = DATABASE_NAME.to_owned() + ":" + VERSION + ":";
    let meta_len = (header.clone() + name).len();
    let metadata = meta_len.to_string() + header.as_str() + name;

    let mut bin: Vec<u8> = encode(metadata);

    match file.write(bin.as_slice()) {
        Ok(n) => {}
        Err(n) => {
            error!("Error when creating database.")
        }
    }

    Ok(())
}

fn create_table(mut file: File, name: &str) {

}