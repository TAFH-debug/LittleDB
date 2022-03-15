use std::io::stdin;

#[derive(Clone)]
pub struct Cmd {
    pub name: String,
    pub func: fn(&[String]) -> ()
}

impl Cmd {
    pub fn new(name: &str, func: fn(&[String]) -> ()) -> Cmd {
        Self::new_a(name.to_string(), func)
    }

    pub fn new_a(name: String, func: fn(&[String]) -> ()) -> Cmd {
        Self {
            name,
            func
        }
    }
}

pub struct CommandHandler {
    pub commands: Vec<Cmd>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        Self {
            commands: vec!()
        }
    }

    pub fn get_cmd(&self, name: String) -> Option<Cmd> {
        for cmd in self.commands.clone() {
            if cmd.name == name {
                return Some(cmd);
            }
        }
        None
    }

    pub fn wait_command(&self) {
        let input_stream = stdin();

        let mut input = String::new();
        input_stream.read_line(&mut input);

        let mut prt = input.trim().split_whitespace();
        let cmd = match prt.next() {
            Some(n) => n,
            None => ""
        };
        let str_slice = &*prt.map(|a| a.to_string()).collect::<Vec<_>>();

        print!("\n");
        match self.get_cmd(cmd.to_string()) {
            Some(n) => (n.func)(str_slice),
            None => println!("Command not found!")
        }
    }
}