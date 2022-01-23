#[warn(non_camel_case_types)]

#[path = "module_interaction/interaction.rs"] mod inter;
#[path = "core/core_binary.rs"] mod bcore;
//#[path = "shell/launcher.rs"] mod launcher;
#[path = "other/error.rs"] mod error;

use std::fs::File;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error, Read};
use crate::inter::{ModuleRequest, Parameter};
use crate::bcore::BCRequestType;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    //launcher::launch(args);
    if false {
        bcore::accept(ModuleRequest::new(BCRequestType::InitDatabase, vec![]));
        bcore::accept(ModuleRequest::new(
            BCRequestType::CreateTable,
            vec!(Parameter::new("", "example_table"))
        ));
    }

    {

    }
    Ok(())
}
