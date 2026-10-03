use core::ffi::{c_int, c_uint};
use std::io::{Error, ErrorKind};

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
#[error("syscall returned non -1 errno: {_0}")]
#[repr(transparent)]
struct InvalidErrno(c_int);

impl From<InvalidErrno> for Error {
    fn from(value: InvalidErrno) -> Self {
        Error::new(ErrorKind::InvalidData, value)
    }
}

#[inline]
pub(crate) fn errnoval(ret: c_int) -> Result<c_uint, Error> {
    Err(match ret {
        -1 => Error::last_os_error(),
        r if r < 0 => InvalidErrno(ret).into(),
        _ => return Ok(ret.try_into().unwrap_or_else(|_| unreachable!())),
    })
}

#[inline]
pub(crate) fn errno(ret: c_int) -> Result<(), Error> {
    errnoval(ret).map(drop)
}
