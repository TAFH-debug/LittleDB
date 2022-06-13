use std::fs::{File};
use std::io::{Read, Write};
use crate::{check_error_e, check_error_f};
use crate::core::{get_code_by_type, Object};
use super::HEADER;

struct DBWriter {
    file: File,

}

impl DBWriter {
    pub fn new(inner: File) -> Self {
        Self {
            file: inner
        }
    }

    pub fn create_db(&mut self) -> Result<(), String> {
        match self.file.read(&mut [0;1]) {
            Ok(0) => (),
            Ok(_) => return Err("File is not empty!".to_string()),
            Err(n) => return Err(n.to_string())
        }
        check_error_e!(self.file.write(HEADER.as_bytes()));
        Ok(())
    }

    pub fn create_table(&mut self, name: String, types: Vec<Type>) -> Result<(), String> {
        let l1 = name.len() as u8;
        let types_bin = {
            let mut res = Vec::new();
            for i in types {
                res.push(get_code_by_type(i));
            }
            res.as_slice()
        };
        let l2 = types_bin.len() as u8;
        let mut all = Vec::new();
        all.push(l1);
        all.append(&mut Vec::from(name.as_bytes()));
        all.push(l2);
        all.append(&mut Vec::from(types_bin));
        all.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]);
        check_error_e!(self.file.write(all.as_slice()));
        Ok(())
    }

    pub fn create_object(&mut self, data: Object, tbl_name: String) -> Result<(), String> {
        todo!()
    }
}