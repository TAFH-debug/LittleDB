use super::{
    encode,
    LDBValue,
    LDBType
};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};
use crate::{
    get_db_file,
    get_tbl_file
};

pub fn insert_values(name: String, values: Vec<LDBValue>) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(crate::get_db_file())
        .unwrap();

    {
        let address: u64 = super::get_storage_address(name);
        match file.seek(SeekFrom::Start(address)) {
            Ok(_) => {},
            Err(_) => return Err("Error when inserting values".to_string())
        }
        let mut data_v: Vec<u8> = Vec::new();
        for i in values {
            match i.vtype {
                LDBType::BOOL => {
                    if i.value.as_str().eq("true") {
                        data_v.push(1)
                    }
                    else if i.value.as_str().eq("false") {
                        data_v.push(0)
                    }
                    else {
                        panic!("Unexpected value.");
                    }
                },
                LDBType::INT => {
                    data_v.append(&mut Vec::from(i.value.parse::<i32>().unwrap().to_be_bytes()));
                },
                LDBType::STRING => {
                    data_v.append(&mut Vec::from(i.value.len().to_be_bytes()));
                    data_v.append(&mut encode(i.value));
                }
            }
        }
        match file.write(&*data_v) {
            Ok(_) => {},
            Err(_) => return Err("Something went wrong".to_string())
        }
    }
    Ok(())
}

pub unsafe fn create_table(name: String, types: String) {
    if name.len() > 255 {
        panic!("Name len is more than 255");
    }
    let meta = name.clone() + &*types;
    let name_len = name.len() as u8;
    let address: u64;

    {
        let filename = get_db_file();
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(filename.clone()).expect("wrong path");

        let meta_len = meta.len();

        let mut bin: Vec<u8> = Vec::new();
        bin.push(meta_len as u8);
        bin.append(&mut encode(meta));

        address = file.seek(SeekFrom::End(1)).unwrap().clone();
        match file.write(bin.as_slice()) {
            Ok(n) => {}
            Err(n) => {
                panic!("{}", "Error write");
            }
        }
    }

    {
        let filename = get_tbl_file();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filename.clone()).unwrap();
        file.seek(SeekFrom::End(0));

        let mut data = Vec::new();
        data.push(name_len);
        data.append(&mut Vec::from(name.clone().as_bytes()));
        data.append(&mut Vec::from(address.to_be_bytes()));

        match file.write(data.as_slice()) {
            Ok(_) => {}
            Err(_) => {
                panic!("{}", "Error write file.");
            }
        }
    }
}