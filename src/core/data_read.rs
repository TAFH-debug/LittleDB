use crate::{
    get_db_file,
    get_tbl_file,
    core::decode
};
use std::{
    io::{Read, Seek, SeekFrom},
    fs::{File, OpenOptions}
};
use std::collections::HashMap;
use std::iter::Map;
use crate::core::{ErrorType, HEADER, LDBObject, OBJECT_SEPARATOR};
use super::{
    LDBValue
};

pub fn read_string_from_file(file: &mut File) -> Result<String, ErrorType> {
    let mut buf = [0; 8];
    file.read(&mut buf);
    let len = usize::from_be_bytes(buf);
    let mut text: Vec<u8> = Vec::new();

    for _ in 0..len {
        let mut buf2 = [0; 1];
        match file.read(&mut buf2) {
            Ok(0) => return Err(ErrorType::UnexpectedEOF),
            Ok(_) => {},
            Err(_) => return Err(ErrorType::IO)
        }
        text.append(&mut Vec::from(buf2));
    }
    decode(text)
}

pub fn is_valid_database() -> Result<bool, ErrorType> {
    let mut file = File::open(get_db_file()).unwrap();
    let mut len = [0; 1];
    let mut header = Vec::new();

    match file.read(&mut len) {
        Ok(_) => {},
        Err(_) => return Err(ErrorType::IO)
    }

    for _ in 0..*len.first().unwrap() {
        let mut buf = [0 as u8; 1];
        file.read(&mut buf);
        header.append(&mut Vec::from(buf));
    }

    let header_s = String::from_utf8(header).unwrap();

    Ok(header_s.as_str() == HEADER)
}

pub fn get_values_binary(name: String) -> Result<(String, Vec<u8>), ErrorType> {
    let address = get_storage_address(name.clone()).unwrap();
    let mut result = Vec::new();
    let header = get_storage_header(name.clone()).unwrap();
    {
        let mut file = File::open(get_db_file()).unwrap();

        file.seek(SeekFrom::Start(address + header.len() as u64 + 1));
        let mut spl = header.split(":");
        spl.next();

        let values = spl.collect::<Vec<&str>>();
        let mut mapped_vals: Vec<(String, String)> = vec!();

        for i in values {
            let mut g = i.split("-");
            mapped_vals.push((
                g.next().unwrap().to_string(),
                g.next().unwrap().to_string()
            ));
        }

        loop {
            let mut check = [0; 1];
            match file.read(&mut check) {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => return Err(ErrorType::IO)
            };
            if *check.first().unwrap() != OBJECT_SEPARATOR { break; }
            result.push(OBJECT_SEPARATOR);
            for i in mapped_vals.clone() {
                match i.1.as_str() {
                    "int" => {
                        let mut buf = [0; 4];
                        match file.read(&mut buf[..]) {
                            Ok(0) => return Err(ErrorType::UnexpectedEOF),
                            Ok(_) => {},
                            Err(_) => return Err(ErrorType::IO)
                        }
                        result.append(&mut Vec::from(buf));
                    },
                    "string" => {
                        let mut buf = [0; 8];
                        match file.read(&mut buf) {
                            Ok(_) => {},
                            Err(_) => return Err(ErrorType::IO)
                        }
                        result.append(&mut Vec::from(buf));
                        for _ in 0..u64::from_be_bytes(buf) {
                            let mut buf3 = [0; 1];
                            match file.read(&mut buf3) {
                                Ok(_) => {},
                                Err(_) => return Err(ErrorType::IO)
                            }
                            result.push(*buf3.first().unwrap());
                        }
                    },
                    "bool" => {
                        let mut buf = [0; 1];
                        match file.read(&mut buf) {
                            Ok(0) => return Err(ErrorType::UnexpectedEOF),
                            Ok(_) => {},
                            Err(_) => return Err(ErrorType::IO)
                        };
                        result.push(*buf.first().unwrap());
                    },
                    _ => return Err(ErrorType::InvalidFormat)
                }
            }
        }
    }
    Ok((header, result))
}

pub fn get_values(name: String) -> Result<Vec<HashMap<String, LDBValue>>, ErrorType> {
    let address = get_storage_address(name.clone()).unwrap();
    let mut result = Vec::new();
    {
        let header = get_storage_header(name.clone()).unwrap();
        let mut file = File::open(get_db_file()).unwrap();

        file.seek(SeekFrom::Start(address + header.len() as u64 + 1));
        let mut spl = header.split(":");
        spl.next();

        let values = spl.collect::<Vec<&str>>();
        let mut mapped_vals: Vec<(String, String)> = vec!();

        for i in 0..values.len() {
            let mut g = values[i].split("-");
            mapped_vals.push((
                g.next().unwrap().to_string(),
                g.next().unwrap().to_string()
            ));
        }

        loop {
            let mut check = [0; 1];
            match file.read(&mut check) {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => return Err(ErrorType::IO)
            };
            if *check.first().unwrap() != OBJECT_SEPARATOR { break; }

            let mut obj: HashMap<String, LDBValue> = HashMap::new();
            for i in mapped_vals.clone() {
                match i.1.as_str() {
                    "int" => {
                        let mut buf = [0; 4];
                        match file.read(&mut buf[..]) {
                            Ok(0) => return Err(ErrorType::UnexpectedEOF),
                            Ok(_) => {},
                            Err(_) => return Err(ErrorType::IO)
                        }

                        let int = i32::from_be_bytes(buf);
                        obj.insert(
                            i.0,
                            LDBValue::INT(int)
                        );
                    },
                    "string" => {
                        let strng = match read_string_from_file(&mut file) {
                            Ok(n) => n,
                            Err(_) => return Err(ErrorType::IO)
                        };
                        obj.insert(
                            i.0,
                            LDBValue::STRING(strng)
                        );
                    },
                    "bool" => {
                        let mut buf = [0; 1];
                        match file.read(&mut buf) {
                            Ok(0) => return Err(ErrorType::UnexpectedEOF),
                            Ok(_) => {},
                            Err(_) => return Err(ErrorType::IO)
                        };
                        let bl = *buf.first().unwrap() == 0u8;
                        obj.insert(
                            i.0,
                            LDBValue::BOOL(bl)
                        );
                    },
                    _ => return Err(ErrorType::InvalidFormat)
                }
            }
            result.push(obj);
        }
    }
    Ok(result)
}

pub fn get_storage_header(name: String) -> Result<String, ErrorType> {
    let mut address: u64 = get_storage_address(name).unwrap();

    let mut file = File::open(get_db_file()).unwrap();

    file.seek(SeekFrom::Start(address));
    let mut buf = [0; 1];
    let mut header: Vec<u8> = Vec::new();

    match file.read(&mut buf[..]) {
        Ok(_) => {},
        Err(_) => return Err(ErrorType::IO)
    };

    for _ in 0..*buf.first().unwrap() {
        let mut buffer: [u8; 1] = [0; 1];
        file.read(&mut buffer);
        header.append(&mut Vec::from(buffer));
    }

    let strin = decode(header).unwrap();
    return Ok(strin);
}

pub fn get_storage_address(storage_name: String) -> Result<u64, ErrorType> {
    let mut metafile = OpenOptions::new()
        .read(true)
        .append(true)
        .open(crate::get_tbl_file())
        .unwrap();
    Ok(loop {
        let mut size = [0 as u8; 1];
        let mut name: Vec<u8> = Vec::new();
        match metafile.read(&mut size) {
            Ok(0) => return Err(ErrorType::StrorageNotFound),
            Ok(_) => (),
            Err(_) => return Err(ErrorType::IO),
        }

        for _ in 0..*size.first().unwrap() as i32 {
            let mut var1 = [0 as u8; 1];
            metafile.read(&mut var1);
            name.push(*var1.first().unwrap())
        }

        if String::from_utf8(name).unwrap() == storage_name {
            let mut buf = [0 as u8; 8];
            metafile.read(&mut buf);
            break u64::from_be_bytes(buf);
        }
        else {
            metafile.seek(SeekFrom::Current(std::mem::size_of::<u64>() as i64));
        }
    })
}