use core::{
    ops::ControlFlow::{self, Break, Continue},
    ptr::read,
};

use crate::const_util::man;

/// NOTE: for drop safety - `B` is [`Copy`] and can't impl [`Drop`]
#[inline]
pub const fn ccontinue_value<B: Copy, C>(this: ControlFlow<B, C>) -> Option<C> {
    match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Continue(ref res) => Some(unsafe { read(res) }),
        &Break(_) => None,
    }
}

/// NOTE: for drop safety - `C` is [`Copy`] and can't impl [`Drop`]
#[inline]
pub const fn cbreak_value<B, C: Copy>(this: ControlFlow<B, C>) -> Option<B> {
    match man!(this) {
        &Continue(_) => None,
        // SAFETY: the original data is ManuallyDrop
        &Break(ref res) => Some(unsafe { read(res) }),
    }
}

/// NOTE: for drop safety - `C` is [`Copy`] and can't impl [`Drop`]
#[inline]
pub const fn cor_break<B: Copy, C>(this: Option<C>, value: B) -> ControlFlow<B, C> {
    match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Some(ref res) => Continue(unsafe { read(res) }),
        &None => Break(value),
    }
}

/// NOTE: for drop safety - `C` is [`Copy`] and can't impl [`Drop`]
#[inline]
pub const fn cor_continue<B, C: Copy>(this: Option<B>, value: C) -> ControlFlow<B, C> {
    match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Some(ref res) => Break(unsafe { read(res) }),
        &None => Continue(value),
    }
}
