use core::ffi::{CStr, c_int};
use std::os::fd::{AsRawFd, BorrowedFd};

use crate::{prelude::*, utils::errno};

// const CLONE_NOFOLLOW: c_int = 0x0001;
const CLONE_NOOWNERCOPY: c_int = 0x0002;

pub(super) fn clonefile(src: &CStr, dst: &CStr) -> Result {
    let clonefile = super::linking::get_clonefile()?;
    let src = src.as_ptr();
    let dst = dst.as_ptr();
    let flags = CLONE_NOOWNERCOPY;
    errno(unsafe { clonefile(src, dst, flags) })
}

pub(super) fn clonefileat(
    srcfd: BorrowedFd<'_>,
    src: &CStr,
    dstfd: BorrowedFd<'_>,
    dst: &CStr,
) -> Result {
    let clonefileat = super::linking::get_clonefileat()?;
    let srcfd = srcfd.as_raw_fd();
    let src = src.as_ptr();
    let dstfd = dstfd.as_raw_fd();
    let dst = dst.as_ptr();
    let flags = CLONE_NOOWNERCOPY;
    errno(unsafe { clonefileat(srcfd, src, dstfd, dst, flags) })
}

pub(super) fn fclonefileat(srcfd: BorrowedFd<'_>, dstfd: BorrowedFd<'_>, dst: &CStr) -> Result {
    let fclonefileat = super::linking::get_fclonefileat()?;
    let srcfd = srcfd.as_raw_fd();
    let dstfd = dstfd.as_raw_fd();
    let dst = dst.as_ptr();
    let flags = CLONE_NOOWNERCOPY;
    errno(unsafe { fclonefileat(srcfd, dstfd, dst, flags) })
}
