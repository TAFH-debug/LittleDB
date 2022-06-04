pub enum Type {
    INT,
    STRING,
    BOOL,
}

pub fn get_type_by_code(code: u8) -> Type {
    match code {
        0 => Type::INT,
        1 => Type::BOOL,
        2 => Type::STRING,
        _ => panic!("Type code is invalid.")
    }
}