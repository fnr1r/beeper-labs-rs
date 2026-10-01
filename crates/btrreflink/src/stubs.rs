#![allow(dead_code)]

use std::io::ErrorKind;

use crate::prelude::*;

pub const SUPPORTS_REFLINKS: Option<bool> = None;

#[inline(always)]
pub(crate) fn unsupported_err() -> Error {
    ErrorKind::Unsupported.into()
}

#[inline(always)]
pub(crate) fn unsupported() -> Result {
    Err(unsupported_err())
}

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

#[inline(always)]
pub fn reflink(_src: FileRef, _dst: FileRef) -> Result {
    unsupported()
}

#[inline(always)]
pub fn reflink_fd(_src: FileRef, _dstfd: DirRef, _dst: &OsStr) -> Result {
    unsupported()
}

#[inline(always)]
pub fn reflink_dd(_srcfd: DirRef, _src: &OsStr, _dstfd: DirRef, _dst: &OsStr) -> Result {
    unsupported()
}

#[inline(always)]
pub fn reflink_pp(_src: &Path, _dst: &Path) -> Result {
    unsupported()
}

#[inline(always)]
pub fn is_reflink_supported() -> bool {
    crate::sys::SUPPORTS_REFLINKS.unwrap_or_default()
}

#[inline(always)]
pub fn can_reflink_pp(_src: &Path, _dst: &Path) -> bool {
    crate::sys::is_reflink_supported()
}

#[inline(always)]
pub fn kernel_copy(_src: FileRef, _dst: FileRef, _len: usize) -> Result {
    unsupported()
}
