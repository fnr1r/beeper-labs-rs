use core::{ffi::c_void, ptr::from_mut};
use std::os::fd::{AsRawFd as _, BorrowedFd};

use rustix::{
    io::Errno,
    ioctl::{Ioctl, IoctlOutput, Opcode, ioctl, opcode::write},
};

use crate::sys::unix::errno::errno;

#[derive(Debug)]
#[repr(C)]
struct FileCloneRange {
    src_fd: i64,
    src_offset: u64,
    src_length: u64,
    dest_offset: u64,
}

unsafe impl Ioctl for FileCloneRange {
    type Output = ();
    const IS_MUTATING: bool = false;
    fn opcode(&self) -> Opcode {
        write::<Self>(0x94, 13)
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
    let op = FileCloneRange {
        src_fd: src.as_raw_fd().into(),
        src_offset,
        src_length,
        dest_offset: dst_offset,
    };
    unsafe { ioctl(dst, op) }
}
