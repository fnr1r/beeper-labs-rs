use core::ops::ControlFlow::{self, Break, Continue};

use crate::const_util::const_map;

/// NOTE: backported from Rust `1.96`
#[inline]
pub const fn break_ok<T, E>(this: ControlFlow<T, E>) -> Result<T, E> {
    const_map! {
        match this;
        [Continue => Err],
        [Break => Ok],
    }
}

/// NOTE: backported from Rust `1.96`
#[inline]
pub const fn continue_ok<T, E>(this: ControlFlow<E, T>) -> Result<T, E> {
    const_map! {
        match this;
        [Continue => Ok],
        [Break => Err],
    }
}
