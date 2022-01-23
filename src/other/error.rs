use std::error::Error;
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug)]
pub struct DataError;

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error when writing file")
    }
}

impl Error for DataError {}

#[macro_export]
macro_rules! error {
    ($($args:expr), *) => {{
        print!("ERROR: ");
        print!("{}", format!( $($args), * ));
        println!(";");
        std::process::exit(0);
    }}
}