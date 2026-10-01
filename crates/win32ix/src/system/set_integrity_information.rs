use core::{ffi::c_void, mem::MaybeUninit};
use std::os::windows::io::AsRawHandle;

use bytemuck::TransparentWrapper;
use windows::Win32::System::Ioctl::{
    FSCTL_SET_INTEGRITY_INFORMATION, FSCTL_SET_INTEGRITY_INFORMATION_BUFFER,
};

use super::ioctl::{
    assume_no_output, cast_self, ioctl, IoctlResult, Opcode, WinDeviceIoControl as Ioctl,
};
use crate::core::Result;

#[derive(Debug, TransparentWrapper)]
#[repr(transparent)]
struct SetIntegrityInformation(FSCTL_SET_INTEGRITY_INFORMATION_BUFFER);

unsafe impl Ioctl for &SetIntegrityInformation {
    type Output = ();
    const IS_MUTATING: bool = false;
    const OPCODE: Opcode = FSCTL_SET_INTEGRITY_INFORMATION;
    fn as_ptr(&self) -> Option<*const c_void> {
        cast_self(self)
    }
    unsafe fn output_from_buf(
        res: IoctlResult,
        extract_output: Option<MaybeUninit<Self::Output>>,
        bytes_returned: u32,
    ) -> Result<Self::Output> {
        assume_no_output(res, extract_output, bytes_returned)
    }
}

pub fn fsctl_set_integrity_information(
    file: &impl AsRawHandle,
    info: &FSCTL_SET_INTEGRITY_INFORMATION_BUFFER,
) -> Result {
    let op = SetIntegrityInformation::wrap_ref(info);
    unsafe { ioctl(file, op) }
}
