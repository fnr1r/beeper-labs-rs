use std::os::windows::io::{AsHandle, BorrowedHandle};

use super::duplicate_extents_to_file::fsctl_duplicate_extents_to_file;
use crate::core::Result;

#[derive(Debug, Clone, Copy)]
pub struct DuplicateExtentsToFileBuilder<'a> {
    src: BorrowedHandle<'a>,
    dst: BorrowedHandle<'a>,
}

impl<'a> DuplicateExtentsToFileBuilder<'a> {
    #[inline]
    pub fn new(src: &'a impl AsHandle, dst: &'a impl AsHandle) -> Self {
        let src = src.as_handle();
        let dst = dst.as_handle();
        Self { src, dst }
    }
    #[allow(clippy::pattern_type_mismatch)]
    #[inline]
    pub fn run(&self, src_offset: i64, dst_offset: i64, src_length: i64) -> Result<()> {
        let Self { src, dst } = self;
        fsctl_duplicate_extents_to_file(src, src_offset, dst, dst_offset, src_length)
    }
}
