#[warn(non_camel_case_types)]

#[path = "core/core.rs"] mod core;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "env/config.rs"] mod config;
#[path = "shell/cmd_shell/shell.rs"] mod shell;

#[cfg(test)]
#[path = "../test/tests.rs"]
mod tests;
mod constants;
pub use constants::*;

use std::io::{Error, stdin};
use crate::core::{insert_values, LDBValue, LDBType, delete_database, is_valid_database, init_database};

fn main() -> Result<(), Error> {
    match launcher::launch() {
        Ok(_) => {},
        Err(_) => println!("Unknown error.")
    }
    delete_database();
    init_database();
    unsafe { 
        core::create_table("users".to_string(), ":int:string".to_string());
        core::create_table("tafh".to_string(), ":int:string:int".to_string());
        core::create_table("admins".to_string(), ":int:bool".to_string());
    }
    {
        let v = vec!(
            LDBValue::new(LDBType::INT, "2".to_string()),
            LDBValue::new(LDBType::STRING, "tafh".to_string()),
            LDBValue::new(LDBType::INT, "3".to_string())
        );
        match insert_values("tafh".to_string(), v) {
            Ok(_) => {},
            Err(n) => panic!("{}", n)
        }
    }
    return Ok(());
}
