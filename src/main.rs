#[warn(non_camel_case_types)]

#[path = "env/env.rs"] mod env;
#[path = "core/core.rs"] mod core;
#[path = "shell/launcher.rs"] mod launcher;
#[path = "env/config.rs"] mod config;
#[path = "shell/cmd_shell/shell.rs"] mod shell;

#[cfg(test)]
#[path = "../test/tests.rs"]
mod tests;
mod constants;

pub use constants::*;
use std::io::{Error, Read, stdin};
use crate::core::{LDBValue};

fn main() -> Result<(), Error> {
    match launcher::launch() {
        Ok(_) => {},
        Err(_) => println!("Unknown error.")
    }
    return Ok(());
}
