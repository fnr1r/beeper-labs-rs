use core::{ffi::c_void, ptr::from_mut};
use std::os::fd::{AsRawFd, BorrowedFd};

use libc::{FICLONERANGE, file_clone_range};
use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl},
};

use crate::utils::errno;

#[derive(Debug)]
#[repr(transparent)]
struct FileCloneRange(file_clone_range);

unsafe impl Ioctl for FileCloneRange {
    type Output = ();
    const IS_MUTATING: bool = false;
    fn opcode(&self) -> Opcode {
        FICLONERANGE as _
    }
    fn as_ptr(&mut self) -> *mut c_void {
        from_mut(self).cast()
    }
    unsafe fn output_from_ptr(
        out: IoctlOutput,
        _extract_output: *mut c_void,
    ) -> Result<Self::Output, Errno> {
        errno(out).map_err(|e| Errno::from_io_error(&e).unwrap())
    }
}

pub(super) fn ioctl_ficlonerange(
    src: BorrowedFd<'_>,
    src_offset: u64,
    dst: BorrowedFd<'_>,
    dst_offset: u64,
    src_length: u64,
) -> Result<(), Errno> {
    let op = FileCloneRange(file_clone_range {
        src_fd: src.as_raw_fd().into(),
        src_offset,
        src_length,
        dest_offset: dst_offset,
    });
    unsafe { ioctl(dst, op) }
}
