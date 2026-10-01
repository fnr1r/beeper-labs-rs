use std::os::windows::io::AsRawHandle;

use windows::Win32::Foundation::HANDLE;

pub(crate) fn as_handle(file: &impl AsRawHandle) -> HANDLE {
    HANDLE(file.as_raw_handle())
}
