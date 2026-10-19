use super::error::unsupported;
use crate::prelude::*;

pub const SUPPORTS_REFLINKS: Option<bool> = None;

#[allow(dead_code)]
pub fn reflink_block(
    _src: FileRef,
    _src_offset: u64,
    _dst: FileRef,
    _dst_offset: u64,
    _src_length: u64,
    _cluster_size: Option<NonZeroU64>,
) -> Result {
    unsupported()
}

#[inline]
pub fn reflink_ff(_src: FileRef, _dst: FileRef) -> Result {
    unsupported()
}

#[inline]
pub fn reflink_fd(_src: FileRef, _dstfd: DirRef, _dst: &OsStr) -> Result {
    unsupported()
}

#[inline]
pub fn reflink_dd(_srcfd: DirRef, _src: &OsStr, _dstfd: DirRef, _dst: &OsStr) -> Result {
    unsupported()
}

#[inline]
pub fn reflink_pp(_src: &Path, _dst: &Path) -> Result {
    unsupported()
}

#[inline]
pub fn is_reflink_supported() -> bool {
    crate::sys::SUPPORTS_REFLINKS.unwrap_or_default()
}

#[inline]
pub fn can_reflink_pp(_src: &Path, _dst: &Path) -> bool {
    crate::sys::is_reflink_supported()
}
