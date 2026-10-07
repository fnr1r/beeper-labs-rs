mod duplicate_extents_to_file;
mod duplicate_extents_to_file_builder;
mod get_integrity_information;
pub mod ioctl;
mod set_integrity_information;
mod set_sparse;
pub mod system_services;

pub mod ops {
    pub use super::{
        duplicate_extents_to_file::fsctl_duplicate_extents_to_file,
        duplicate_extents_to_file_builder::DuplicateExtentsToFileBuilder,
        get_integrity_information::fsctl_get_integrity_information,
        set_integrity_information::fsctl_set_integrity_information, set_sparse::fsctl_set_sparse,
    };
}
