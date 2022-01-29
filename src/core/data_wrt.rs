use std::fs::OpenOptions;
use super::encode;
use std::io::{SeekFrom, Seek, Write};
use crate::error;
use std::path::{Path, PathBuf};

pub unsafe fn create_table(name: String, types: String) {
    let meta = name + &*types;
    let address: u64;
    let filename = format!("{}/{}.{}",
                           crate::FOLDER_PATH.with(|a| a.clone().take()),
                           crate::DB_NAME.with(|a| a.clone().take()),
                           crate::MAIN_FILE_EXTENSION);
    {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(filename.clone()).expect("wrong path");

        let meta_len = meta.len();

        let mut bin: Vec<u8> = Vec::new();
        bin.push(meta_len as u8);
        bin.append(&mut encode(meta));

        address = file.seek(SeekFrom::End(0)).unwrap().clone();
        match file.write(bin.as_slice()) {
            Ok(n) => {}
            Err(n) => {
                error!("{}", "Error write");
            }
        }
    }

    {
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .open(filename.clone()).unwrap();
        file.seek(SeekFrom::End(0));
        match file.write(address.to_be_bytes().as_slice()) {
            Ok(_) => {}
            Err(_) => {
                error!("{}", "Error address")
            }
        }
    }
}