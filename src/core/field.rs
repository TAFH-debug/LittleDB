#[derive(PartialEq, Debug)]
pub enum Field {
    Int(i32),
    String(String),
    Bool(bool)
}

#[derive(Debug)]
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

pub fn get_code_by_type(_type: Field) -> u8 {
    match _type {
        Field::Int(_) => 0,
        Field::Bool(_) => 1,
        Field::String(_) => 2
    }
}

pub fn compare_types(obj: Object, tbl: Vec<Field>) -> bool {
    if obj.fields.len() != tbl.len() { return false; }
    let mut i = 0;
    for t in obj.fields {
        if t != tbl[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub fn get_type_by_code(code: u8) -> Field {
    match code {
        0 => Field::Int(0),
        1 => Field::Bool(false),
        2 => Field::String("".to_string()),
        _ => panic!("Type code is invalid.")
    }
}