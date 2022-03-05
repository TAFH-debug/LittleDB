mod cmd_shell_core;
use std::{
    io::{stdin, Split}
};
use cmd_shell_core::*;

fn exit(a: &[String]) {
    std::process::exit(0);
}

fn help(a: &[String]) {
    println!("Unimplemented!");
}


pub fn start_shell() {
    let mut commands = vec!();
    commands.push(Cmd::new("exit", exit));
    commands.push(Cmd::new("help", help));
    let mut handler = CommandHandler::new();

    handler.commands = commands;
    loop {
        "";
        print!(">");
        handler.wait_command();
    }
}

