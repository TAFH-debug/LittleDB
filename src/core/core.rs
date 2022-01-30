pub mod address_manager;
pub mod data_wrt;
pub mod data_read;

pub use address_manager::*;
pub use data_wrt::*;
pub use data_read::*;

use crate::{inter, StorageType};

pub struct ObjectMetadata {
    segment_offset: u16,
    offset: u16,
    size: u32
}

pub struct StorageMetadata {
    segment_offset: u16,
    offset: u16,
    size: u32,
    objects: Vec<ObjectMetadata>,
    tbl_type: StorageType
}

pub struct DatabaseMetadata {
    segment_offset: u16,
    offset: u16,
    size: u32,
    tables: Vec<StorageMetadata>
}