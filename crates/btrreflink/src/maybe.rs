#![allow(clippy::absolute_paths)]

use core::ops::ControlFlow;

use super::maybewrap::implwrap;
use crate::prelude::*;

#[inline]
pub fn reflink_ff(src: FileRef, dst: FileRef) -> ControlFlow<Result> {
    implwrap(crate::sys::reflink_ff(src, dst))
}

#[inline]
pub fn reflink_fd(srcfd: FileRef, dstfd: DirRef, dst: &OsStr) -> ControlFlow<Result> {
    implwrap(crate::sys::reflink_fd(srcfd, dstfd, dst))
}

#[inline]
pub fn reflink_dd(
    srcfd: DirRef,
    src: &OsStr,
    dstfd: DirRef,
    dst: &OsStr,
) -> ControlFlow<Result<()>> {
    implwrap(crate::sys::reflink_dd(srcfd, src, dstfd, dst))
}

#[inline]
pub fn reflink_pp(src: &Path, dst: &Path) -> ControlFlow<Result> {
    implwrap(crate::sys::reflink_pp(src, dst))
}

#[inline]
pub fn kernel_copy(src: FileRef, dst: FileRef, len: usize) -> ControlFlow<Result> {
    implwrap(crate::sys::kernel_copy(src, dst, len))
}
