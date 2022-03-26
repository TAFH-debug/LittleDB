use std::fs::File;
use std::io::Read;
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};
use crate::constants::{get_db_file, get_tbl_file};
use crate::core::core::*;
use crate::core::data_read::get_storage_address;
use crate::core::util::encode;

pub fn delete_database() {
    File::create(get_db_file()).unwrap();
    File::create(get_tbl_file()).unwrap();
}

pub fn init_database() -> Result<(), ()> {
    let mut file = File::create(get_db_file()).unwrap();

    let mut header = Vec::new();
    header.push(HEADER.len() as u8);
    header.append(&mut Vec::from(HEADER.as_bytes()));
    file.write(&*header);
    Ok(())
}

pub fn _insert_values(name: String, values: Vec<LDBValue>) -> Result<(), ErrorType> {
    let mut data_v: Vec<u8> = Vec::new();
    data_v.push(OBJECT_SEPARATOR);

    for i in values {
        match i {
            LDBValue::BOOL(n) => {
                if n.to_string().eq("true") {
                    data_v.push(1)
                } else if n.to_string().eq("false") {
                    data_v.push(0)
                } else {
                    return Err(ErrorType::InvalidArgument);
                }
            }
            LDBValue::INT(n) => {
                data_v.append(&mut Vec::from(n.to_be_bytes()));
            }
            LDBValue::STRING(n) => {
                data_v.append(&mut Vec::from(n.len().to_be_bytes()));
                data_v.append(&mut encode(n));
            }
        }
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(crate::constants::get_db_file())
        .unwrap();

    match skip_to_values(&mut file, name.clone()) {
        Ok(_) => {}
        Err(n) => return Err(n),
    }

    let mut prev_data = Vec::new();
    file.read_to_end(&mut prev_data);

    skip_to_values(&mut file, name);
    data_v.append(&mut prev_data);

    match file.write(&*data_v) {
        Ok(_) => {}
        Err(_) => return Err(ErrorType::IO),
    }

    Ok(())
}

pub fn skip_to_values(file: &mut File, name: String) -> Result<(), ErrorType> {
    let address = get_storage_address(name).unwrap();

    match file.seek(SeekFrom::Start(address)) {
        Ok(_) => (),
        Err(_) => return Err(ErrorType::IO),
    }

    let mut buf = [0; 1];

    match file.read(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err(ErrorType::IO),
    };

    file.seek(SeekFrom::Current(*buf.first().unwrap() as i64));

    Ok(())
}

pub fn _create_table(name: String, types: String) -> Result<(), ErrorType> {
    if name.len() > 255 || name.len() < 1 {
        return Err(ErrorType::InvalidArgument);
    }
    let meta = name.clone() + &*types;
    let name_len = name.len() as u8;
    let address: u64;

    {
        let filename = get_db_file();
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(filename.clone())
            .expect("wrong path");

        let meta_len = meta.len();

        let mut bin: Vec<u8> = Vec::new();
        bin.push(meta_len as u8);
        bin.append(&mut encode(meta));

        address = file.seek(SeekFrom::End(0)).unwrap();
        match file.write(bin.as_slice()) {
            Ok(_) => {}
            Err(_) => return Err(ErrorType::IO),
        }
    }

    {
        let filename = get_tbl_file();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(filename.clone())
            .expect("Cannot open database file.");
        file.seek(SeekFrom::End(0));

        let mut data = Vec::new();
        data.push(name_len);
        data.append(&mut Vec::from(name.clone().as_bytes()));
        data.append(&mut Vec::from(address.to_be_bytes()));

        match file.write(data.as_slice()) {
            Ok(_) => {}
            Err(_) => return Err(ErrorType::IO),
        }
    }
    Ok(())
}
