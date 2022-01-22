mod core;
mod error;
mod data_manager;
mod launcher;
mod help;

use std::fs::File;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error, Read};
use crate::error::*;
use crate::core::decode;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    launcher::launch(args);
    //TEST MODE
    {
        let mut file = "data.txt";
        data_manager::create_database(file, "Users");
        data_manager::create_table(file, "example", ":int:string:");
    }

    {
        let mut file = File::open("data.txt")?;
        let mut bin: Vec<u8> = Vec::new();
        file.read_to_end(&mut bin);

        println!("{}", decode(bin));
    }

    Ok(())
}
