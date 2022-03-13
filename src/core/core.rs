pub mod address_manager;
pub mod data_wrt;
pub mod data_read;

pub use address_manager::*;
pub use data_wrt::*;
pub use data_read::*;

use crate::StorageType;
#[derive(Debug)]
pub struct LDBValue {
    vtype: LDBType,
    value: String
}

#[derive(Debug)]
pub enum LDBType {
    STRING,
    INT,
    BOOL
}

impl LDBValue {
    pub fn new(vtype: LDBType, value: String) -> LDBValue {
        Self {
            vtype,
            value
        }
    }
}

pub struct StorageMetadata {
    segment_offset: u16,
    offset: u16,
    size: u32,
    tbl_type: StorageType
}

pub struct DatabaseMetadata {
    segment_offset: u16,
    offset: u16,
    size: u32,
    tables: Vec<StorageMetadata>
}