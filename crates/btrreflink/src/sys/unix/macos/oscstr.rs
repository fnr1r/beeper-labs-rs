use core::ffi::{CStr, FromBytesWithNulError};
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    ffi::{CString, OsStr},
    io::{Error, Result},
    os::unix::ffi::OsStrExt as _,
};

pub(super) fn cstr(path: &OsStr) -> Result<Cow<'_, CStr>> {
    let path = path.as_bytes();
    Ok(match CStr::from_bytes_with_nul(path) {
        Ok(res) => Borrowed(res),
        Err(FromBytesWithNulError::NotNulTerminated) => Owned(CString::new(path).unwrap()),
        Err(e) => return Err(Error::other(e)),
    })
}
