pub enum Type {
    INT,
    STRING,
    BOOL,
}

pub enum Field {
    Int(i32),
    String(String),
    Bool(bool)
}

pub struct Object {
    fields: Vec<Field>,
}

impl Object {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields
        }
    }
}

pub fn get_code_by_type(_type: Type) -> u8 {
    match _type {
        Type::INT => 0,
        Type::BOOL => 1,
        Type::STRING => 2
    }
}

pub fn get_type_by_code(code: u8) -> Type {
    match code {
        0 => Type::INT,
        1 => Type::BOOL,
        2 => Type::STRING,
        _ => panic!("Type code is invalid.")
    }
}