use core::ffi::{c_char, c_int};

pub(super) type CInt = c_int;
pub(super) type XStr = *const c_char;

pub(super) type Clonefile = unsafe extern "C" fn(XStr, XStr, CInt) -> CInt;
pub(super) type ClonefileAt = unsafe extern "C" fn(CInt, XStr, CInt, XStr, CInt) -> CInt;
pub(super) type FClonefileAt = unsafe extern "C" fn(CInt, CInt, XStr, CInt) -> CInt;
