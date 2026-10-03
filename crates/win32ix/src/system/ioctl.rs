use core::{
    ffi::c_void,
    mem::{size_of, MaybeUninit},
    ptr::from_ref,
};
use std::os::windows::io::AsRawHandle;

pub use windows::Win32::System::Ioctl::{
    FSCTL_GET_INTEGRITY_INFORMATION_BUFFER as FsctlGetIntegrityInformation,
    FSCTL_SET_INTEGRITY_INFORMATION_BUFFER as FsctlSetIntegrityInformation,
};
use windows::Win32::System::IO::DeviceIoControl;

use crate::{core::Result, utils::as_handle};

pub type Opcode = u32;
pub type IoctlResult = Result;

/// A trait defining the properties of an `ioctl` command. (for Windows)
///
/// # Safety
///
/// See [`rustix` docs]. Basically make sure that the output buffer is
/// initialized and the `ioctl` didn't fail spectacularly and stuff.
///
/// [`rustix` docs]: https://docs.rs/rustix/latest/rustix/ioctl/trait.Ioctl.html
pub unsafe trait WinDeviceIoControl {
    type Output;
    const OPCODE: Opcode;
    const IS_MUTATING: bool;
    fn as_ptr(&self) -> Option<*const c_void>;
    #[allow(clippy::missing_safety_doc)]
    unsafe fn output_from_buf(
        res: IoctlResult,
        extract_output: Option<MaybeUninit<Self::Output>>,
        bytes_returned: u32,
    ) -> Result<Self::Output>;
}

/// Perform an `ioctl` call.
///
/// # Safety
///
/// [`rustix` docs](https://docs.rs/rustix/latest/rustix/ioctl/fn.ioctl.html)
#[inline]
pub unsafe fn ioctl<I: WinDeviceIoControl>(
    handle: &impl AsRawHandle,
    ioctl: I,
) -> Result<I::Output> {
    let handle = as_handle(handle);
    let request = I::OPCODE;
    let input = ioctl.as_ptr();
    let input_len = size_of::<I>() as u32;
    let (mut output_buf, output_len) = if I::IS_MUTATING {
        let buf = MaybeUninit::<I::Output>::uninit();
        (Some(buf), size_of::<I::Output>() as u32)
    } else {
        (None, 0)
    };
    let output = output_buf.as_mut().map(|e| e.as_mut_ptr() as _);
    let mut bytes_returned = 0u32;

    let res = unsafe {
        DeviceIoControl(
            handle,
            request,
            input,
            input_len,
            output,
            output_len,
            Some(&mut bytes_returned),
            None,
        )
    };

    unsafe { I::output_from_buf(res, output_buf, bytes_returned) }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn cast_self<T: WinDeviceIoControl>(this: &T) -> Option<*const c_void> {
    Some(from_ref(this) as _)
}

pub(crate) fn assume_output<T>(
    res: IoctlResult,
    extract_output: Option<MaybeUninit<T>>,
    bytes_returned: u32,
) -> Result<T> {
    assert_eq!(bytes_returned as usize, size_of::<T>());
    let Some(buf) = extract_output else {
        unreachable!();
    };
    res.map(|()| unsafe { buf.assume_init() })
}

pub(crate) fn assume_no_output(
    res: IoctlResult,
    extract_output: Option<MaybeUninit<()>>,
    bytes_returned: u32,
) -> Result {
    assert_eq!(bytes_returned as usize, 0);
    let None = extract_output else {
        unreachable!();
    };
    res
}
