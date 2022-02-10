mod cmd_shell_core;

use std::{
    io::{stdin, Split}
};
use cmd_shell_core::*;

fn exit(a: &[String]) {
    std::process::exit(0);
}

fn help(a: &[String]) {
    write("Unimplemented!");
}

pub fn start_shell() {
    let input_stream = stdin();
    let mut commands = vec!();
    commands.push(Cmd::new("exit", exit));
    commands.push(Cmd::new("help", help));
    let mut input = String::new();
    input_stream.read_line(&mut input);

    let mut prt = input.trim().split_whitespace();
    let cmd = prt.next().unwrap();

    if cmd == command.name.as_str() {
        let f = command.func;
        f(&prt.map(|a| a.to_string()).collect::<Vec<_>>()[..]);
    }
}

