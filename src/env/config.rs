use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error, Read};
use json::*;

extern crate json;

pub fn load() {
    if std::path::Path::new(crate::CONFIG_FILE).exists() {
        let mut file = File::open(crate::CONFIG_FILE).expect("Error when reading configs.");
        let mut reader = BufReader::new(file);
        let mut str = String::new();
        reader.read_to_string(&mut str);
        let deser = parse(&*str).expect("Invalid configs");
        //TODO use values
    }
    else {
        let mut file = File::create(crate::CONFIG_FILE).expect("Error when creating configs");

        let default_configs = object! {
            recent_dbs: []
        };

        write!(file, "{}", default_configs.pretty(4));
    }
}