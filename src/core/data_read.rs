use crate::{
    get_db_file,
    get_tbl_file,
    core::decode
};
use std::{
    io::{Read, Seek, SeekFrom},
    fs::{File, OpenOptions}
};
use super::{
    LDBType,
    LDBValue
};

pub fn isvalid_database() {

}

pub fn get_values(name: String) -> Result<Vec<LDBValue>, String> {
    let address = get_storage_address(name.clone());
    let mut result = Vec::new();
    {
        let header = get_storage_header(name.clone()).unwrap();
        let mut file = File::open(get_tbl_file()).unwrap();

        file.seek(SeekFrom::Start(address - 1 + get_storage_header_size(name) as u64));
        let mut spl = header.split(":");
        spl.next();
        for i in spl {
            match i {
                "int" => {
                    let mut buf = [0; 4];
                    file.read(&mut buf[..]);
                    result.push(LDBValue::new(LDBType::INT, i32::from_be_bytes(buf).to_string()));
                    println!("{:#?}", buf);
                    println!("{}", i32::from_be_bytes(buf));
                },
                "string" => {
                    let mut buf = [0; 8];
                    file.read(&mut buf[..]);
                    let len = usize::from_be_bytes(buf);
                    println!("{}", len);
                    let mut text: Vec<u8> = Vec::new();
                    for i in 0..len {
                        let mut buf2 = [0; 1];
                        file.read(&mut buf2[..]);
                        text.append(&mut Vec::from(buf2));
                    }

                    println!("{}", String::from_utf8(text).unwrap());
                },
                "bool" => panic!("Unimplemented!"),
                _ => {
                    panic!("{} is not allowed here.", i)
                }
            }
            println!("{}", i);
        }
        
    }
    Err("AA".to_string())
}

pub fn get_storage_header_size(name: String) -> u8 {
    let address = get_storage_address(name);

    if address != 0 {
        let mut file = File::open(get_db_file()).unwrap();
        
        file.seek(SeekFrom::Start(address - 1));

        let mut buf = [0; 1];
        let mut header: Vec<u8> = Vec::new();

        file.read(&mut buf[..]);

        return *buf.first().unwrap()
    }
    0
}

pub fn get_storage_header(name: String) -> Option<String> {
    let mut address: u64 = 0;
    {
        let mut file = File::open(get_tbl_file()).unwrap();

        loop {
            let mut name_size = [0; 1];
            match file.read(&mut name_size) {
                Ok(0) => break,
                Ok(n) => (),
                Err(n) => panic!("Something went wrong")
            }
            let mut comp_name = String::new();

            for i in 0..*name_size.first().unwrap() {
                let mut buf: [u8; 1] = [0; 1];
                match file.read(&mut buf) {
                    Ok(n) => (),
                    Err(n) => panic!("Something went wrong")
                }
                comp_name += &*String::from_utf8(Vec::from(buf)).unwrap();
            }
            if comp_name == name {
                let mut address_r: [u8; 8] = [0; 8];
                file.read(&mut address_r);
                address = u64::from_be_bytes(address_r);
                break;
            }
            else {
                file.seek(SeekFrom::Current(8));
            }
        }
    }
    
    if address != 0 {
        let mut file = File::open(get_db_file()).unwrap();
        
        file.seek(SeekFrom::Start(address - 1));

        let mut buf = [0; 1];
        let mut header: Vec<u8> = Vec::new();

        file.read(&mut buf[..]);

        for i in 0..*buf.first().unwrap() {
            let mut buffer: [u8; 1] = [0; 1];
            file.read(&mut buffer);
            header.append(&mut Vec::from(buffer));
        }

        let strin = decode(header).unwrap();
        return Some(strin);
    }
    None
}

pub fn get_storage_address(storage_name: String) -> u64 {
    let mut metafile = OpenOptions::new()
        .read(true)
        .append(true)
        .open(crate::get_tbl_file())
        .unwrap();
    loop {
        let mut size = [0 as u8; 1];
        let mut name: Vec<u8> = Vec::new();
        match metafile.read(&mut size) {
            Ok(0) => panic!("EOF is not expected here."),
            Ok(_) => (),
            Err(_) => panic!("Unknown error."),
        }

        //Read name
        for i in 0..*size.first().unwrap() as i32 {
            let mut var1 = [0 as u8; 1];
            metafile.read(&mut var1);
            name.push(*var1.first().unwrap())
        }

        //If storage name equals to name that we received returns storage address.
        if String::from_utf8(name).unwrap() == storage_name {
            let mut buf = [0 as u8; 8];
            metafile.read(&mut buf);
            break u64::from_be_bytes(buf);
        }
        else {
            metafile.seek(SeekFrom::Current(std::mem::size_of::<u64>() as i64));
        }
    }
}