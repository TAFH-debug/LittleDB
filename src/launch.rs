use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::error;

struct Flags {}
impl Flags {
    const MODE: &'static str = "-m";
    const HELP: &'static str = "-h";
    const PORT: &'static str = "-p";

    fn flag_type(val: &str) -> FlagType {
        match val {
            Flags::MODE => FlagType::MODE,
            Flags::HELP => FlagType::HELP,
            Flags::PORT => FlagType::PORT,
            _ => FlagType::ERROR
        }
    }
}

#[derive(PartialEq)]
pub enum FlagType {
    HELP,
    ERROR,
    FILENAME,
    MODE,
    PORT
}

impl FlagType {
    fn should_have_arg(&self) -> bool {
        match self {
            FlagType::HELP | FlagType::ERROR => false,
            _ => true
        }
    }
}

pub struct Flag {
    value: String,
    ftype: FlagType
}

impl Flag {
    fn new(val: String, ftype: FlagType) -> Flag {
        Flag {
            value: val,
            ftype
        }
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({})", self.value)
    }
}

/*
Gets flags and check filename.
*/
pub fn get_flags(input: Vec<String>) -> Vec<Flag> {
    let mut result = Vec::new();

    //first arg is the .exe filename
    let is_help = false; //input.len() < 2 && Flags::flag_type(&*input.clone()[1]) == FlagType::HELP;

    if is_help {
        result.push(Flag::new("".to_string(), FlagType::HELP));
        return result;
    }

    //if input.clone().len() < 2 {
    //    error!("File name was not specified.")
    //}

    let filename = match input.last().cloned() {
        Some(i) => i,
        None => error!("File name was not specified.")
    };

    result.push(Flag::new(filename.to_string(), FlagType::FILENAME));
    
    let mut after = input;
    after.remove(0);
    after.remove(after.len() - 1);

    let mut is_arg = false;
    let mut last_tp: FlagType = FlagType::ERROR;
    for i in after {

        if is_arg {
            result.push(Flag::new(i, last_tp));
            is_arg = false;
            last_tp = FlagType::ERROR;
            continue;
        }

        let tp = Flags::flag_type(&*i);
        if tp.should_have_arg() {
            last_tp = tp;
            is_arg = true;
        }
        else {
            result.push(Flag::new("".to_string(), tp));
        }
    }
    result
}

fn print_help() {
    println!("

    ")
}