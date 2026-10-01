use core::{ffi::c_void, mem::MaybeUninit};
use std::os::windows::io::AsRawHandle;

use windows::{
    core::Result,
    Win32::System::Ioctl::{
        FSCTL_GET_INTEGRITY_INFORMATION, FSCTL_GET_INTEGRITY_INFORMATION_BUFFER,
    },
};

use super::ioctl::{assume_output, ioctl, IoctlResult, Opcode, WinDeviceIoControl as Ioctl};

#[derive(Debug)]
#[repr(transparent)]
struct GetIntegrityInformation;

unsafe impl Ioctl for GetIntegrityInformation {
    type Output = FSCTL_GET_INTEGRITY_INFORMATION_BUFFER;
    const IS_MUTATING: bool = true;
    const OPCODE: Opcode = FSCTL_GET_INTEGRITY_INFORMATION;
    fn as_ptr(&self) -> Option<*const c_void> {
        None
    }
    unsafe fn output_from_buf(
        res: IoctlResult,
        extract_output: Option<MaybeUninit<Self::Output>>,
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
