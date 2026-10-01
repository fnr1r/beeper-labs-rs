use std::os::windows::fs::MetadataExt;

use win32ix::{
    core::Result,
    storage::filesystem::{FileAttributeSparseFile, FileFlagsAndAttributes},
    system::ops::fsctl_set_sparse,
};

use super::{blocked::reflink_block_internal, chkcluster::get_checked_cluster_size};
use crate::prelude::*;

fn is_sparse(attrs: u32) -> bool {
    (FileFlagsAndAttributes(attrs) & FileAttributeSparseFile).0 != 0
}

fn round_up(num_to_round: i64, multiple: i64) -> i64 {
    debug_assert!(multiple > 0);
    debug_assert_eq!((multiple & (multiple - 1)), 0);
    (num_to_round + multiple - 1) & -multiple
}

pub(super) fn reflink_inner(src: FileRef, dst: FileRef) -> Result {
    let src_metadata = src.metadata()?;
    let src_file_size = src_metadata.file_size();
    let src_attributes = src_metadata.file_attributes();
    let src_is_sparse = is_sparse(src_attributes);
    fsctl_set_sparse(dst, true)?;
    let src_cluster_size = get_checked_cluster_size(src, dst)?;

    // file_size must be sufficient to hold the data.
    // TODO test if the current implementation works:
    // Later on, we round up the bytes to copy in order to end at a cluster boundary.
    // This might very well result in us cloning past the file end.
    // Let's hope windows api sanitizes this, because otherwise a clean implementation is not really possible.
    dst.set_len(src_file_size)?;

    // We must end at a cluster boundary
    let total_copy_len = if let Some(src_cluster_size) = src_cluster_size {
        // Round to the next cluster size
        round_up(src_file_size.try_into().unwrap(), src_cluster_size as _)
    } else {
        src_file_size.try_into().unwrap()
    };

    reflink_block_internal(src, 0, dst, 0, total_copy_len as u64, src_cluster_size)?;
    if !src_is_sparse {
        fsctl_set_sparse(dst, false)?;
    }

    Ok(dst.set_permissions(src_metadata.permissions())?)
}
