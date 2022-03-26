use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub const OBJECT_SEPARATOR: u8 = 0x00;
pub const HEADER: &'static str = "littledb:v0.1";

#[derive(Debug)]
pub enum ErrorType {
    StrorageNotFound,
    UnexpectedEOF,
    IO,
    InvalidArgument,
    InvalidFormat,
    TypeMismatch,
}

#[derive(Debug)]
pub struct LDBObject {
    values: HashMap<String, LDBValue>,
}

#[derive(Debug)]
pub enum LDBValue {
    STRING(String),
    INT(i32),
    BOOL(bool),
}

impl LDBValue {
    pub fn eq_type(&self, other: &LDBValue) -> bool {
        self.to_string() == other.to_string()
    }
}

impl ToString for LDBValue {
    fn to_string(&self) -> String {
        match self {
            LDBValue::INT(_) => "int".to_string(),
            LDBValue::STRING(_) => "string".to_string(),
            LDBValue::BOOL(_) => "bool".to_string(),
        }
    }
}
