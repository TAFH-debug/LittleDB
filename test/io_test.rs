use std::io::{BufReader, BufRead, Read};
use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

pub fn main() {
    let filename = format!("{}/{}.{}",
                           crate::FOLDER_PATH.with(|a| a.take()),
                           crate::DB_NAME.with(|a| a.take()),
                           crate::MAIN_FILE_EXTENSION);
    let mut buffer = Vec::new();
    println!("{}", filename);
    File::open(filename.clone()).unwrap().read_to_end(&mut buffer.clone());
    println!("{}", crate::bcore::decode(buffer));
}