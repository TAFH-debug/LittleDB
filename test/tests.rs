use std::{fs::File, io::Read};

use super::*;
use crate::core::decode;

pub fn test_crtbg() {
    let mut file = File::open(get_db_file()).unwrap();
    let mut buf = [0; 1];

    while !buf.is_empty() {
        let mut header: Vec<u8> = Vec::new();

        match file.read(&mut buf[..]) {
            Ok(0) => break,
            _ => {}
        }
        for i in 0..*buf.first().unwrap() {
            let mut buffer: [u8; 1] = [0; 1];
            file.read(&mut buffer);
            header.append(&mut Vec::from(buffer));
        }
        let strin = decode(header).unwrap();
        if strin.split(":").take(1).next().unwrap() == "tafh".to_string() {
            println!("{}", strin);
        }
    }
}

pub fn test_crtb() {
    let mut file = File::open(get_db_file()).unwrap();
    let mut buf = [0; 1];

    while !buf.is_empty() {
        let mut header: Vec<u8> = Vec::new();

        match file.read(&mut buf[..]) {
            Ok(0) => break,
            _ => {}
        }
        for i in 0..*buf.first().unwrap() {
            let mut buffer: [u8; 1] = [0; 1];
            file.read(&mut buffer);
            header.append(&mut Vec::from(buffer));
        }
        println!("{}", decode(header).unwrap());
    }
}

#[test]
pub fn test_create_table() {
    let mut file = File::open(get_db_file()).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    println!("{}", crate::core::decode(buffer).unwrap());
}