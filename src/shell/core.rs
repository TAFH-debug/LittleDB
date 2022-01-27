use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::os::windows::fs::FileExt;
use crate::error;

pub enum Keychar {
    SEMICOLON(String),
    COMMA(String)
}

pub enum LDBType {
    TEXT,
    INTEGER,
    FLOAT,
    BOOL,
    LONG,
    DOUBLE
}

struct LDBTable {
    types: Vec<LDBType>,
    object: Vec<usize>
}

impl LDBType {
    fn get_type(from: &str) -> LDBType {
        match from {
            "text" => LDBType::TEXT,
            "bool" => LDBType::BOOL,
            "float" => LDBType::FLOAT,
            "int" => LDBType::INTEGER,
            "double" => LDBType::DOUBLE,
            "long" => LDBType::LONG,
            _ => panic!("This type not found.")
        }
    }
}

trait ToText {
    fn to_text(&self) -> &str;
}

impl ToText for Vec<LDBType> {
    fn to_text(&self) -> &str {
        let mut a = "".to_string();
        for i in self {
            match i {
                LDBType::TEXT => a = a.clone() + ":text",
                LDBType::BOOL => a = a.clone() + ":bool",
                LDBType::FLOAT => a = a.clone() + ":float",
                LDBType::INTEGER => a = a.clone() + ":int",
                LDBType::DOUBLE => a = a.clone() + ":double",
                LDBType::LONG => a = a.clone() + ":long",
                _ => {}
            }
        }
        ""
    }
}

//pub fn read_table(path: &str) -> LDBTable {
//
//}

//TODO error handling
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
//TODO