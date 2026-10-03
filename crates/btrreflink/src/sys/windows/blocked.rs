use std::fs::File;

use win32ix::{core::Result, system::ops::DuplicateExtentsToFileBuilder as Builder};

use super::{iter8ors::iter_add, refs::ClusterSize};

// Must be smaller than 4GB; This is always a multiple of ClusterSize
const MAX_BLOCK: u64 = 4 * super::GIB as u64;

#[inline(always)]
fn assert_mod(cluster_size: impl Into<u64>, bytes_copied: u64, bytes_to_copy: u64) {
    let cluster_size = cluster_size.into();
    debug_assert_eq!(bytes_copied % cluster_size, 0);
    debug_assert_eq!(bytes_to_copy % cluster_size, 0);
}

#[allow(clippy::pattern_type_mismatch)]
pub(super) fn reflink_block_internal(
    from: &File,
    from_offset: u64,
    to: &File,
    to_offset: u64,
    src_length: u64,
    cluster_size: Option<ClusterSize>,
) -> Result {
    let builder = Builder::new(from, to);

    let max_io_size = MAX_BLOCK - cluster_size.unwrap_or_default() as u64;

    let while_fn = |bytes_copied: u64| bytes_copied < src_length;
    let add_fn = |bytes_copied: u64| max_io_size.min(src_length - bytes_copied);
    let add_fn = |bytes_copied: u64| {
        let bytes_to_copy = add_fn(bytes_copied);
        if let Some(cluster_size) = cluster_size {
            assert_mod(cluster_size, bytes_copied, bytes_to_copy);
        }
        bytes_to_copy
    };
    let offset_fn = |bytes_copied: u64| (from_offset + bytes_copied, to_offset + bytes_copied);
    let map_fn = |(bytes_copied, bytes_to_copy)| {
        let (src_offset, dst_offset) = offset_fn(bytes_copied);
        (src_offset, dst_offset, bytes_to_copy)
    };
    let try_fn = |(src_offset, dst_offset, bytes_to_copy)| {
        #[allow(clippy::cast_possible_wrap)]
        builder.run(src_offset as _, dst_offset as _, bytes_to_copy as _)
    };

    iter_add(0, |bytes_copied: &_| add_fn(*bytes_copied))
        .take_while(|(bytes_copied, _)| while_fn(*bytes_copied))
        .map(map_fn)
        .try_for_each(try_fn)
}
