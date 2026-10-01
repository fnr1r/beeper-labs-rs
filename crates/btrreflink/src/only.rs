use crate::prelude::*;

#[inline(always)]
pub fn reflink_block(
    mut src: impl AsFileRef,
    src_offset: u64,
    mut dst: impl AsFileRef,
    dst_offset: u64,
    src_length: u64,
    cluster_size: Option<NonZeroU64>,
) -> Result {
    crate::sys::reflink_block(
        src.as_file(),
        src_offset,
        dst.as_file(),
        dst_offset,
        src_length,
        cluster_size,
    )
}

#[inline(always)]
pub fn reflink(mut src: impl AsFileRef, mut dst: impl AsFileRef) -> Result {
    crate::sys::reflink(src.as_file(), dst.as_file())
}

#[inline(always)]
pub fn reflink_fd(
    mut src: impl AsFileRef,
    mut dstfd: impl AsDirRef,
    dst: impl AsFileName,
) -> Result {
    let src = src.as_file();
    let dstfd = dstfd.as_dir();
    let dst = dst.as_ref();
    crate::sys::reflink_fd(src, dstfd, dst)
}

#[inline(always)]
pub fn reflink_dd(
    mut srcfd: impl AsDirRef,
    src: impl AsFileName,
    mut dstfd: impl AsDirRef,
    dst: impl AsFileName,
) -> Result {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let srcfd = srcfd.as_dir();
    let dstfd = dstfd.as_dir();
    crate::sys::reflink_dd(srcfd, src, dstfd, dst)
}

#[inline(always)]
pub fn reflink_pp(src: impl AsFilePath, dst: impl AsFilePath) -> Result {
    let src = src.as_ref();
    let dst = dst.as_ref();
    crate::sys::reflink_pp(src, dst)
}

#[inline(always)]
pub fn is_reflink_supported() -> bool {
    crate::sys::is_reflink_supported()
}

#[inline(always)]
pub fn can_reflink_pp(src: impl AsFilePath, dst: impl AsFilePath) -> bool {
    let src = src.as_ref();
    let dst = dst.as_ref();
    crate::sys::can_reflink_pp(src, dst)
}

#[inline(always)]
pub fn kernel_copy(mut src: impl AsFileRef, mut dst: impl AsFileRef, len: usize) -> Result {
    crate::sys::kernel_copy(src.as_file(), dst.as_file(), len)
}
