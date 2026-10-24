use core::{ffi::c_void, mem::MaybeUninit, ptr::null};
use std::os::windows::io::AsRawHandle;

use windows::Win32::System::Ioctl::{
    FSCTL_GET_INTEGRITY_INFORMATION, FSCTL_GET_INTEGRITY_INFORMATION_BUFFER,
};

use super::ioctl::{
    assume_output, ioctl, IoControlCode, IoControlOutput, WinDeviceIoControl as Ioctl,
};
use crate::core::Result;

#[derive(Debug)]
#[repr(transparent)]
struct GetIntegrityInformation;

unsafe impl Ioctl for GetIntegrityInformation {
    type Output = FSCTL_GET_INTEGRITY_INFORMATION_BUFFER;
    const IS_MUTATING: bool = true;
    const OPCODE: IoControlCode = FSCTL_GET_INTEGRITY_INFORMATION;
    fn as_ptr(&self) -> *const c_void {
        null()
    }
    unsafe fn output_from_buf(
        res: IoControlOutput,
        extract_output: MaybeUninit<Self::Output>,
        bytes_returned: u32,
    ) -> Result<Self::Output> {
        assume_output(res, extract_output, bytes_returned)
    }
}

pub fn fsctl_get_integrity_information(
    file: &impl AsRawHandle,
) -> Result<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER> {
    unsafe { ioctl(file, GetIntegrityInformation) }
}
