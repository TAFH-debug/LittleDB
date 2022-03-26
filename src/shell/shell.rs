use crate::core::core::{LDBValue, ErrorType};
use crate::core::data_wrt::{delete_database, init_database};
use crate::net;
use std::io::{stdout, Write};
use super::shell_core::*;

fn exit(_: &[String]) {
    std::process::exit(0);
}

fn help(_: &[String]) {
    println!("Unimplemented!");
}

fn create_table(args: &[String]) {
    let mut vec_args = Vec::from(args);

    let table_name = vec_args.remove(0);
    let types = vec_args.remove(0);
    unsafe {
        crate::core::data_wrt::_create_table(table_name, types);
    }
}

fn listen(args: &[String]) {
    unsafe {
        net::listener::listen(format!("127.0.0.1:{}", *crate::constants::PORT.lock().unwrap()));
    }
}

fn insert_values(args: &[String]) {
    let mut vec_args = Vec::from(args);
    let table_name = vec_args.remove(0);
    let temp = vec_args.remove(0);
    let splitted: Vec<&str> = temp.split(":").collect::<Vec<&str>>();

    let mut values = Vec::new();

    for i in splitted {
        let tv = i.split_once("-").unwrap();
        match tv.0 {
            "int" => values.push(LDBValue::INT(tv.1.parse::<i32>().unwrap())),
            "string" => values.push(LDBValue::STRING(tv.1.to_string())),
            "bool" => values.push(LDBValue::BOOL(tv.1.parse::<bool>().unwrap())),
            _ => println!("Type {} doesn't exists.", tv.0),
        }
    }

    match crate::env::env::insert_values(table_name, values) {
        Ok(_) => {}
        Err(ErrorType::TypeMismatch) => println!("Type mismatch."),
        Err(_) => println!("unknown error"),
    }
}

fn get_values(args: &[String]) {
    let data = crate::core::data_read::get_values(args.first().unwrap().to_string()).unwrap();
    println!("{:#?}", data);
}

fn restart_db(_: &[String]) {
    delete_database();
    init_database();
    println!("Done.");
}

pub fn start_shell() {
    let mut commands = vec![];
    let mut handler = CommandHandler::new();

    commands.push(Cmd::new("listen", listen));
    commands.push(Cmd::new("exit", exit));
    commands.push(Cmd::new("help", help));
    commands.push(Cmd::new("create", create_table));
    commands.push(Cmd::new("insert", insert_values));
    commands.push(Cmd::new("get", get_values));
    commands.push(Cmd::new("restart", restart_db));

    handler.commands = commands;
    loop {
        print!("> ");
        stdout().flush();
        handler.wait_command();
    }
}
