#[warn(non_camel_case_types)]

#[path = "module_interaction/interaction.rs"] mod inter;
#[path = "core/core.rs"] mod core;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "other/error.rs"] mod error;
#[path = "env/config.rs"] mod config;
#[path = "shell/shell.rs"] mod shell;
#[path = "../test/tests.rs"] mod tests;

#[cfg(test)]
#[path = "../test/tests.rs"]
mod tests;

mod constants;
pub use constants::*;

use std::fs::File;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error, Read};

fn main() -> Result<(), Error> {
    launcher::launch();
    unsafe {
        //core::create_table("users".to_string(), ":int:string".to_string());
        //core::create_table("tafh".to_string(), ":int:string:int".to_string());
        //core::create_table("admins".to_string(), ":int:bool".to_string());
    }
    {
        tests::test_crtb();
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp);
        println!("{}", core::find_storage(inp.trim().to_string()).unwrap());
    }
    return Ok(());
}
