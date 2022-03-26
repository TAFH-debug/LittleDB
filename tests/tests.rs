use std::{fs::File, io::Read};

use crate::core::decode;

#[test]
pub fn test_create_table() {
    let mut file = File::open().unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    println!("{}", decode(buffer).unwrap());
}