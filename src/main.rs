#[warn(non_camel_case_types)]

#[path = "core/core.rs"] mod core;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "other/error.rs"] mod error;
#[path = "env/config.rs"] mod config;
#[path = "shell/cmd_shell/shell.rs"] mod shell;

#[cfg(test)]
#[path = "../test/tests.rs"]
mod tests;
mod constants;
pub use constants::*;


use std::io::{ Error };
use crate::core::{insert_values, LDBValue, LDBType};

fn main() -> Result<(), Error> {
    launcher::launch();
    unsafe { 
        //core::create_table("users".to_string(), ":int:string".to_string());
        //core::create_table("tafh".to_string(), ":int:string:int".to_string());
        //core::create_table("admins".to_string(), ":int:bool".to_string());
    }
    {
        let v = vec!(
            LDBValue::new(LDBType::INT, "2".to_string()),
            LDBValue::new(LDBType::STRING, "tafh".to_string()),
            LDBValue::new(LDBType::INT, "3".to_string())
        );
        insert_values("tafh".to_string(), v);
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp);
        core::get_values(inp.trim().to_string());
        //println!("{}", core::get_storage_header(inp.trim().to_string()).unwrap());
    }
    return Ok(());
}
