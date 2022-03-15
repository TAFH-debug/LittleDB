mod cmd_shell_core;
use cmd_shell_core::*;

fn exit(_: &[String]) {
    std::process::exit(0);
}

fn help(_: &[String]) {
    println!("Unimplemented!");
}

fn create_table(args: &[String]) {
    let mut vec_args = Vec::from(args);
    let table_name = match args.first() {
        Some(n) => n,
        None => panic!("Required argument `table_name` that is missing.")
    };
    vec_args.remove(0);
    unsafe {
        crate::core::create_table(table_name.clone().to_string(), vec_args.first().unwrap().clone().to_string());
    }
}

fn insert_values(args: &[String]) {
    let mut vec_args = Vec::from(args);
    let table_name = vec_args.remove(0);
    table_name;
}


pub fn start_shell() {
    let mut commands = vec!();
    let mut handler = CommandHandler::new();

    commands.push(Cmd::new("exit", exit));
    commands.push(Cmd::new("help", help));
    commands.push(Cmd::new("crtbl", create_table));

    handler.commands = commands;
    loop {
        print!(">");
        handler.wait_command();
    }
}

