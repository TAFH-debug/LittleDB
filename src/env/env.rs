use crate::core::core::{ErrorType, LDBValue};
use crate::core::data_read::get_storage_header;

pub fn create_table(name: String, fields: Vec<(String, LDBValue)>) -> Result<(), ErrorType> {
    let mut str_types = String::new();
    for (j, i) in fields {
        str_types += &*format!(":{}-{}", j, i.to_string());
    }
    unsafe { crate::core::data_wrt::_create_table(name, str_types) }
}

pub fn get_table_types(name: String) -> Result<Vec<LDBValue>, ErrorType> {
    let header = match get_storage_header(name) {
        Ok(n) => n,
        Err(n) => return Err(n),
    };

    let mut splitted = header.split(":");
    splitted.next();
    let mut result = Vec::new();
    for i in splitted.collect::<Vec<&str>>() {
        let tpl = i.split_once("-").unwrap();
        match tpl.1 {
            "int" => result.push(LDBValue::INT(0)),
            "string" => result.push(LDBValue::STRING("".to_string())),
            "bool" => result.push(LDBValue::BOOL(true)),
            _ => return Err(ErrorType::InvalidFormat),
        }
    }
    Ok(result)
}

pub fn insert_values(name: String, values: Vec<LDBValue>) -> Result<(), ErrorType> {
    let types = get_table_types(name.clone()).unwrap();

    if values.len() != types.len() {
        return Err(ErrorType::TypeMismatch);
    }

    for it in types.iter().zip(values.iter()) {
        let (i, j) = it;
        if !i.eq_type(j) {
            return Err(ErrorType::TypeMismatch);
        }
    }

    crate::core::data_wrt::_insert_values(name, values)
}
