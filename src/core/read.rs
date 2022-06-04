use std::fs::File;
use std::io::Read;
use crate::check_error_f;
use crate::core::get_type_by_code;
use super::field::Type;

struct TableReader<'a> {
    name: String,
    types: Vec<Type>,
    lenght: u64,
    file: &'a File
}

impl<'a> TableReader<'a> {
    fn new(name: String, types: Vec<Type>, lenght: u64, file: &'a File) -> Self {
        Self {
            name,
            types,
            lenght,
            file
        }
    }
}

struct DBReader {
    file: File,
    header: String
}

impl DBReader {
    pub fn load(&mut self) -> Result<(), String> {
        let mut buf = [0; super::HEADER.len()];

        let res = match self.file.read(&mut buf) {
            Ok(0) => Err("Unexpected EOF".to_string()),
            Ok(_) => Ok(()),
            Err(n) => Err(n.to_string())
        };
        self.header = String::from_utf8(Vec::from(buf)).unwrap();
        res
    }

    pub fn new(inner: File) -> DBReader {
        Self {
            header: String::new(),
            file: inner
        }
    }

    pub fn read_table(&mut self) -> Result<TableReader, String> {
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
            match self.file.read(&mut buf) {
                Ok(_) => (),
                Err(n) => return Err(n.to_string()),
            }
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
        match self.file.read(&mut buf) {
            Ok(0) => return Err("Unexpected EOF".to_string()),
            Ok(_) => (),
            Err(n) => return Err(n.to_string()),
        }
        Ok(*buf.first().unwrap())
    }
}