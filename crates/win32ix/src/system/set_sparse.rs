use core::{ffi::c_void, mem::MaybeUninit};
use std::os::windows::io::AsRawHandle;

use windows::Win32::System::Ioctl::FSCTL_SET_SPARSE;

use super::ioctl::{
    assume_no_output, cast_self, ioctl, IoControlCode, IoControlOutput, WinDeviceIoControl as Ioctl,
};
use crate::core::Result;

#[derive(Debug)]
#[repr(transparent)]
struct SetSparse(u32);

unsafe impl Ioctl for SetSparse {
    type Output = ();
    const IS_MUTATING: bool = false;
    const OPCODE: IoControlCode = FSCTL_SET_SPARSE;
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

pub fn fsctl_set_sparse(file: &impl AsRawHandle, b: bool) -> Result {
    unsafe { ioctl(file, SetSparse(b.into())) }
}
