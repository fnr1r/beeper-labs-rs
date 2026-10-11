use std::fs::File;

pub use self::altfd::{AsDirRef, AsFileRef, DirRef, FileRef};
use self::{
    blocked::reflink_block_internal, full::reflink_inner, refs::ClusterSize,
    support::check_reflink_support,
};
use crate::prelude::*;
pub use crate::stubs::{SUPPORTS_REFLINKS, is_reflink_supported, reflink_dd, reflink_fd};

mod altfd;
mod blocked;
mod chkcluster;
mod full;
mod iter8ors;
mod refs;
mod support;

const KIB: u16 = 1024;
const MIB: u32 = 1024 * KIB as u32;
const GIB: u32 = 1024 * MIB;

pub fn reflink_block(
    src: FileRef,
    src_offset: u64,
    dst: FileRef,
    dst_offset: u64,
    src_length: u64,
    cluster_size: Option<NonZeroU64>,
) -> Result<()> {
    Ok(reflink_block_internal(
        src,
        src_offset,
        dst,
        dst_offset,
        src_length,
        Some(ClusterSize::try_from(cluster_size)?),
    )?)
}

pub fn reflink_ff(src: FileRef, dst: FileRef) -> Result<()> {
    Ok(reflink_inner(src, dst)?)
}

pub fn reflink_pp(src: &Path, dst: &Path) -> Result<()> {
    let mut src = File::open(src)?;
    let mut dst = File::create_new(dst)?;
    reflink_ff(&mut src, &mut dst)
}

pub fn can_reflink_pp(src: &Path, dst: &Path) -> bool {
    check_reflink_support(src, dst).unwrap_or_default()
}
