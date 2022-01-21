mod core;
mod error;
mod data_manager;
mod launch;

use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, Read};
use crate::error::*;
use crate::data_manager::decode;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    {
        let mut file = File::create("data.txt")?;
        data_manager::create_database(file, "Users");
    }

    {
        let mut file = File::open("data.txt")?;
        let mut bin: Vec<u8> = Vec::new();
        file.read_to_end(&mut bin);

        println!("{}", decode(bin));
    }

    return Ok(());


    let path = "data";
    {
        let mut file = File::create(path)?;
        let header_size = 9;
        let header = "data:v0.1".to_string();
        let name_size = 12;
        let name = "users".to_string();

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(header_size);
        bytes.append(&mut header.into_bytes());
        bytes.push(name_size);
        bytes.append(&mut name.into_bytes());

        for i in bytes {
            file.write(&[i << 1]);
        }
    }

    {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer);

        let header_size = buffer.first().unwrap() >> 1;
        let mut header = "".to_string();
        for i in 1..header_size+1 {
            header += &*match String::from_utf8(vec!(buffer[i as usize] >> 1)) {
                Ok(n) => {
                    n
                }
                Err(n) => {
                    println!("{}", buffer[i as usize]);
                    "a".to_string()
                }
            }
        }
        println!("{}", header);
    }
    Ok(())
}
