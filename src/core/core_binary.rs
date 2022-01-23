pub mod address_manager;
pub mod request_manager;
pub use address_manager::*;
pub use request_manager::*;
use crate::inter;

pub enum StorageType {
    TABLE,
    SINGLE_OBJECTS
}

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