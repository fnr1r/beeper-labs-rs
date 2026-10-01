use core::ffi::{CStr, FromBytesWithNulError};
use std::{
    borrow::Cow::{self, *},
    ffi::{CString, OsStr},
    io::{Error, Result},
    os::unix::ffi::OsStrExt,
};

pub(super) fn cstr<'a>(path: &'a OsStr) -> Result<Cow<'a, CStr>> {
    let path = path.as_bytes();
    Ok(match CStr::from_bytes_with_nul(path) {
        Ok(res) => Borrowed(res),
        Err(FromBytesWithNulError::NotNulTerminated) => Owned(CString::new(path).unwrap()),
        Err(e) => return Err(Error::other(e)),
    })
}
