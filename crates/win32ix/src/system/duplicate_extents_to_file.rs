use core::{ffi::c_void, mem::MaybeUninit};
use std::os::windows::io::AsRawHandle;

use windows::Win32::System::Ioctl::{DUPLICATE_EXTENTS_DATA, FSCTL_DUPLICATE_EXTENTS_TO_FILE};

use super::ioctl::{
    assume_no_output, cast_self, ioctl, IoControlCode, IoControlOutput, WinDeviceIoControl as Ioctl,
};
use crate::{core::Result, utils::as_handle};

#[derive(Debug)]
#[repr(transparent)]
struct DuplicateExtentsToFile(DUPLICATE_EXTENTS_DATA);

unsafe impl Ioctl for DuplicateExtentsToFile {
    type Output = ();
    const IS_MUTATING: bool = false;
    const OPCODE: IoControlCode = FSCTL_DUPLICATE_EXTENTS_TO_FILE;
    fn as_ptr(&self) -> *const c_void {
        cast_self(self)
    }
    unsafe fn output_from_buf(
        res: IoControlOutput,
        extract_output: MaybeUninit<Self::Output>,
        bytes_returned: u32,
    ) -> Result<Self::Output> {
        assume_no_output(res, extract_output, bytes_returned)
    }
}

pub fn fsctl_duplicate_extents_to_file(
    src: &impl AsRawHandle,
    src_offset: i64,
    dst: &impl AsRawHandle,
    dst_offset: i64,
    src_length: i64,
) -> Result {
    let data = DuplicateExtentsToFile(DUPLICATE_EXTENTS_DATA {
        FileHandle: as_handle(src),
        SourceFileOffset: src_offset,
        TargetFileOffset: dst_offset,
        ByteCount: src_length,
    });
    unsafe { ioctl(dst, data) }
}
