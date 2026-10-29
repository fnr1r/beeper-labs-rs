use core::ffi::CStr;
use std::{
    borrow::Cow::{self, Borrowed, Owned},
    ffi::{
        CString,
        FromBytesWithNulError::{self, InteriorNul},
        OsStr, OsString,
    },
    os::windows::io::AsRawHandle,
};

use widestring::U16CStr;

use crate::{
    core::{Error, HResult, PCStr, PCWStr, Result},
    foundation::{Handle, ERROR_INVALID_PARAMETER},
};

pub(crate) fn as_handle(file: &impl AsRawHandle) -> Handle {
    Handle(file.as_raw_handle())
}

pub(crate) trait U16CStrExt {
    fn as_internal(&self) -> PCWStr;
}

impl U16CStrExt for U16CStr {
    fn as_internal(&self) -> PCWStr {
        PCWStr(self.as_ptr())
    }
}

pub(crate) trait CStrExt {
    fn as_internal(&self) -> PCStr;
}

impl CStrExt for CStr {
    fn as_internal(&self) -> PCStr {
        PCStr(self.as_ptr().cast())
    }
}

fn err_invalid_param(message: impl AsRef<str>) -> Error {
    Error::new(HResult::from(ERROR_INVALID_PARAMETER), message)
}

fn cstring_trunc_error() -> Error {
    err_invalid_param("returned string not null-terminated")
}

pub(crate) fn cstring_checked(mut buf: Vec<u8>) -> Result<CString> {
    let nul_pos = buf.iter().position(|e| e == &0).map(|e| e + 1);
    buf.truncate(nul_pos.ok_or_else(cstring_trunc_error)?);
    Ok(unsafe { CString::from_vec_with_nul_unchecked(buf) })
}

/// # Safety
///
/// Must not contain null bytes.
unsafe fn osstr_null_terminatex(s: &OsStr) -> CString {
    let mut buf = Vec::with_capacity(s.len() + 1);
    buf.extend(s.as_encoded_bytes());
    buf.push(b'\0');
    unsafe { CString::from_vec_with_nul_unchecked(buf) }
}

const CONTAINS_NULL: &str = "passed an OsStr containing non-terminator null-bytes";

fn err_contains_null_at(pos: usize) -> Error {
    err_invalid_param(format!("{CONTAINS_NULL} at: {pos}"))
}

pub(crate) fn osstr_null_terminate(s: &OsStr) -> Result<Cow<'_, CStr>> {
    Ok(match CStr::from_bytes_with_nul(s.as_encoded_bytes()) {
        Ok(s) => Borrowed(s),
        Err(FromBytesWithNulError::NotNulTerminated) => Owned(unsafe { osstr_null_terminatex(s) }),
        Err(InteriorNul { position }) => {
            return Err(err_contains_null_at(position));
        }
    })
}

fn null_truncate(data: &mut Vec<u8>, inclusive: bool) {
    if let Some(pos) = data.iter().position(|c| c == &0) {
        data.truncate(pos + usize::from(inclusive));
    }
}

pub(crate) unsafe fn osstring_from_null_truncate(mut buf: Vec<u8>, inclusive: bool) -> OsString {
    null_truncate(&mut buf, inclusive);
    unsafe { OsString::from_encoded_bytes_unchecked(buf) }
}
