#[cfg(unix)]
use std::io::Error;

#[cfg(unix)]
pub(crate) fn errno(ret: core::ffi::c_int) -> Result<(), Error> {
    if ret == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}
