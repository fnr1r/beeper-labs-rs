use std::{fs::File, os::fd::AsFd as _};

use rustix::fs::ioctl_ficlone;

use self::{clonerange::ioctl_ficlonerange, ufile::UnixFileExt as _};
use crate::prelude::*;

mod clonerange;
mod ufile;

#[inline]
pub fn reflink_block(
    src: FileRef,
    src_offset: u64,
    dst: FileRef,
    dst_offset: u64,
    src_length: u64,
    _cluster_size: Option<NonZeroU64>,
) -> Result<()> {
    let src = src.as_fd();
    let dst = dst.as_fd();
    Ok(ioctl_ficlonerange(
        src, src_offset, dst, dst_offset, src_length,
    )?)
}

#[inline]
pub fn reflink_ff(src: FileRef, dst: FileRef) -> Result<()> {
    Ok(ioctl_ficlone(src, dst)?)
}

pub fn reflink_fd(src: FileRef, dstfd: DirRef, dst: &OsStr) -> Result<()> {
    let mut dst = File::create_new_at(dstfd, dst)?;
    reflink_ff(src, &mut dst)
}

pub fn reflink_dd(srcfd: DirRef, src: &OsStr, dstfd: DirRef, dst: &OsStr) -> Result<()> {
    let mut src = File::read_at(srcfd, src)?;
    reflink_fd(&mut src, dstfd, dst)
}

pub fn reflink_pp(src: &Path, dst: &Path) -> Result<()> {
    let mut src = File::open(src)?;
    let mut dst = File::create_new(dst)?;
    reflink_ff(&mut src, &mut dst)
}
