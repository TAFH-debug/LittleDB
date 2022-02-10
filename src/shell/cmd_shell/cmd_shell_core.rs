pub struct Cmd {
    pub name: String,
    pub func: fn(&[String]) -> ()
}

impl Cmd {
    pub fn new(name: &str, func: fn(&[String]) -> ()) -> Cmd {
        Self {
            name: name.to_string(),
            func
        }
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
    pub fn get_cmd(&self, name: String) -> Option<Cmd> {
        for cmd in self.commands {
            if cmd.name == name {
                return Some(cmd);
            }
        }
        None
    }
}

pub fn write(arg: &str) {
    println!("{}", arg);
    print!(">")
}

pub fn