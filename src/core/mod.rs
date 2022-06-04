mod write;
mod read;
mod field;

pub use field::*;
pub use write::*;
pub use read::*;

const HEADER: &'static str = "littledb:0.1.0";