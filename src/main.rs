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

    Ok(())
}
