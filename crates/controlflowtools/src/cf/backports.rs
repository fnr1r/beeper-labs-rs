use core::{
    ops::ControlFlow::{self, Break, Continue},
    ptr::read,
};

use crate::const_util::man;

/// NOTE: backported from Rust `1.96`
#[inline]
pub const fn into_value<T>(this: ControlFlow<T, T>) -> T {
    match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Continue(ref res) | &Break(ref res) => unsafe { read(res) },
    }
}
