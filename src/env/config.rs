use std::collections::HashMap;
use std::fs::{File};
use std::io::{Write, BufReader, Read};
use json::*;

extern crate json;

pub fn load() {
    if std::path::Path::new(crate::CONFIG_FILE).exists() {
        let file = File::open(crate::CONFIG_FILE).expect("Error when reading configs.");
        let mut reader = BufReader::new(file);
        let mut str = String::new();
        reader.read_to_string(&mut str);
        //let deser = parse(&*str).expect("Invalid configs");
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