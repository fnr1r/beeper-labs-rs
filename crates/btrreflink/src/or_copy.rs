#![allow(clippy::absolute_paths)]

use core::ops::ControlFlow::{self, Continue};

use controlflowtools::{exts::CfBranchExt as _, result::branch_fwd};

use crate::{
    fallback::{fs_copy, io_copy},
    prelude::*,
};

fn reflink_or_copy_ff_inner(src: FileRef, dst: FileRef) -> Result {
    crate::maybe::reflink_ff(src, dst).c_to_b(|()| io_copy(src, dst, None))
}

#[inline]
pub fn reflink_or_copy_ff(mut src: impl AsFileRef, mut dst: impl AsFileRef) -> Result {
    reflink_or_copy_ff_inner(src.as_file(), dst.as_file())
}

fn reflink_or_copy_pp_inner(src: &Path, dst: &Path) -> Result {
    crate::maybe::reflink_pp(src, dst).c_to_b(|()| fs_copy(src, dst, None))
}

#[inline]
pub fn reflink_or_copy_pp(src: impl AsFilePath, dst: impl AsFilePath) -> Result {
    reflink_or_copy_pp_inner(src.as_ref(), dst.as_ref())
}

#[inline]
fn file_len(file: &File) -> Result<u64> {
    file.metadata().map(|e| e.len())
}

#[inline]
fn try_reflink_or_krnl(src: FileRef, dst: FileRef) -> ControlFlow<Result, u64> {
    crate::maybe::reflink_ff(src, dst)?;
    let len = branch_fwd(file_len(src)).map_break(Err)?;
    // TODO: remove kernel_copy. Rust's standard library already does that
    // TODO: better error handling
    crate::maybe::kernel_copy(src, dst, len.try_into().unwrap())?;
    Continue(len)
}

fn reflink_or_krnl_or_copy_inner(src: FileRef, dst: FileRef) -> Result {
    try_reflink_or_krnl(src, dst).c_to_b(|len| io_copy(src, dst, Some(len)))
}

#[inline]
pub fn reflink_or_krnl_or_copy(mut src: impl AsFileRef, mut dst: impl AsFileRef) -> Result {
    reflink_or_krnl_or_copy_inner(src.as_file(), dst.as_file())
}

/*#[cfg(unix)]
pub fn reflink_or_copy_at(src: &mut File, dst: &mut File) -> Result {
    maybe_reflink(src, dst).c_to_b(|()| copy(src, dst, file_len(src)?))
}*/

/*#[inline]
pub fn xreflink_or_copy<P: AsRef<Path>, Q: AsRef<Path>>(
    src: Utf8File<P>,
    dst: Utf8File<Q>,
) -> Result {
    let mut src_file = src.open()?;
    let mut dst_file = dst.create()?;
    reflink_or_copy(&mut src_file, &mut dst_file)
}*/
