use std::{
    borrow::Cow::{self, Borrowed, Owned},
    ffi::{
        FromBytesWithNulError::{self, InteriorNul, NotNulTerminated},
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

fn osstr_try_as_null_terminated(s: &OsStr) -> Result<&OsStr, FromBytesWithNulError> {
    Err(match s.as_encoded_bytes().iter().position(|e| e == &0) {
        Some(pos) if pos == s.len() => return Ok(s),
        Some(position) => InteriorNul { position },
        None => NotNulTerminated,
    })
}

fn osstr_null_terminatex(s: &OsStr) -> OsString {
    let mut buf = OsString::with_capacity(s.len() + 1);
    buf.push(s);
    buf.push("\0");
    buf
}

fn err_invalid_param(message: impl AsRef<str>) -> Error {
    Error::new(HResult::from(ERROR_INVALID_PARAMETER), message)
}

const CONTAINS_NULL: &str = "passed an OsStr containing non-terminator null-bytes";

fn err_contains_null_at(pos: usize) -> Error {
    err_invalid_param(format!("{CONTAINS_NULL} at: {pos}"))
}

pub(crate) fn osstr_null_terminate(s: &OsStr) -> Result<Cow<'_, OsStr>> {
    Ok(match osstr_try_as_null_terminated(s) {
        Ok(s) => Borrowed(s),
        Err(FromBytesWithNulError::NotNulTerminated) => Owned(osstr_null_terminatex(s)),
        Err(InteriorNul { position }) => {
            return Err(err_contains_null_at(position));
        }
    })
}

pub(crate) trait OsStrExt {
    fn as_internal(&self) -> PCStr;
}

impl OsStrExt for OsStr {
    fn as_internal(&self) -> PCStr {
        PCStr(self.as_encoded_bytes().as_ptr())
    }
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

pub(crate) trait U16CStrExt {
    fn as_internal(&self) -> PCWStr;
}

impl U16CStrExt for U16CStr {
    fn as_internal(&self) -> PCWStr {
        PCWStr(self.as_ptr())
    }
}
