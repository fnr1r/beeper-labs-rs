#![allow(clippy::absolute_paths)]

use controlflowtools::exts::CfBranchExt as _;

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
