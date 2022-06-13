use std::fs::File;
use std::io::Read;
use crate::{check_error_e, check_error_f};
use crate::core::{Field, get_type_by_code, Object};

struct TableReader<'a> {
    name: String,
    types: Vec<Field>,
    lenght: u64,
    file: &'a File
}

impl<'a> TableReader<'a> {
    fn new(name: String, types: Vec<Field>, lenght: u64, file: &'a File) -> Self {
        Self {
            name,
            types,
            lenght,
            file
        }
    }

    pub fn read_object(&mut self) -> Result<Object, String> {
        let mut res = Vec::new();
        for i in self.types {
            let temp = match i {
                Field::Int(_) => {
                    let mut buf = [0; 4];
                    check_error_e!(self.file.read(&mut buf));
                    Field::Int(i32::from_be_bytes(buf))
                },
                Field::String(_) => {
                    let mut size_b = [0; 4];
                    check_error_e!(self.file.read(&mut size_b));
                    let size = u32::from_be_bytes(size_b);
                    let mut buf = Vec::new();
                    for _ in 0..size as usize {
                        let mut buf2 = [0; 1];
                        check_error_e!(self.file.read(&mut buf));
                        buf.push(*buf2.first().unwrap());
                    }
                    Field::String(String::from_utf8(buf).unwrap())
                },
                Field::Bool(_) => {
                    let mut buf = [0; 1];
                    check_error_e!(self.file.read(&mut buf));
                    Field::Bool(*buf.first().unwrap() != 0)
                }
            };
            res.push(temp);
        }
        Ok(Object::new(res))
    }
}

struct DBReader {
    file: File,
    header: String,
    is_loaded: bool
}

impl DBReader {
    pub fn load(&mut self) -> Result<(), String> {
        let mut buf = [0; super::HEADER.len()];

        check_error_e!(self.file.read(&mut buf));
        self.header = String::from_utf8(Vec::from(buf)).unwrap();
        self.is_loaded = true;
        Ok(())
    }

    pub fn new(inner: File) -> Self {
        Self {
            header: String::new(),
            file: inner,
            is_loaded: false
        }
    }

    pub fn read_table(&mut self) -> Result<TableReader, String> {
        if !self.is_loaded { return Err("Reader is not loaded!".to_string()) }
        let l1 = match self.read() {
            Ok(n) => n,
            Err(n) => return Err(n)
        };

        let name = {
            let temp = check_error_f!(self.read_n(l1 as u32));

            match String::from_utf8(temp) {
                Ok(n) => n,
                Err(_) => return Err("string decoding error".to_string())
            }
        };

        let l2 = check_error_f!(self.read());

        let types = {
            let mut res = Vec::new();
            let mut mbuf = check_error_f!(self.read_n(l2 as u32));
            for i in mbuf {
                res.push(get_type_by_code(i));
            }
            res
        };

        let length = {
            let mut buf = [0; 8];
            check_error_e!(self.file.read(&mut buf));
            u64::from_be_bytes(buf)
        };

        Ok(TableReader::new(name, types, length, &self.file))
    }

    pub fn read_n(&mut self, count: u32) -> Result<Vec<u8>, String> {
        let mut mbuf = Vec::new();

        for _ in 0..count {
            match self.read() {
                Err(n) => return Err(n),
                Ok(n) => mbuf.push(n),
            };
        }
        Ok(mbuf)
    }

    pub fn read(&mut self) -> Result<u8, String> {
        let mut buf = [0; 1];
        check_error_e!(self.file.read(&mut buf));
        Ok(*buf.first().unwrap())
    }
}