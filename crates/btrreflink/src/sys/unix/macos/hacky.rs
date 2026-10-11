use core::mem::forget;
use std::{
    fs::remove_file,
    io::ErrorKind,
    os::fd::{AsFd as _, AsRawFd as _, FromRawFd as _, OwnedFd},
};

use rustix::{
    fs::{Mode, OFlags, fcntl_getfl, open},
    io::dup2,
};

use super::fcntls::fcntl_getpath;
use crate::prelude::*;

/// Hacky way to implement fd-to-fd reflink on macos
pub fn reflink_ff(src: FileRef, dst: FileRef) -> Result {
    let dst_fd = dst.as_fd();
    let dst_path = fcntl_getpath(dst_fd)?;
    let mut oflags = fcntl_getfl(dst_fd)?;
    oflags.set(OFlags::CREATE | OFlags::TRUNC | OFlags::EXCL, false);

    let dst_dirname = dst_path.parent().ok_or(ErrorKind::InvalidInput)?;
    let dst_filename = dst_path.file_name().ok_or(ErrorKind::InvalidInput)?;

    remove_file(&dst_path)?;
    let oflags_dir = OFlags::RDONLY | OFlags::DIRECTORY;
    let dst_dir_fd = open(dst_dirname, oflags_dir, Mode::empty())?;
    super::reflink_fd(src, dst_dir_fd.as_fd(), dst_filename)?;

    let dst_fd_new = open(dst_path, oflags, Mode::empty())?;
    let mut dst_fd_old = unsafe { OwnedFd::from_raw_fd(dst.as_raw_fd()) };
    dup2(dst_fd_new, &mut dst_fd_old)?;
    forget(dst_fd_old);

    Ok(())
}
