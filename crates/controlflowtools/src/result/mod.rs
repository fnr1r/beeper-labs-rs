use core::ops::ControlFlow::{self, Break, Continue};

pub use self::{
    backports::{break_ok, continue_ok},
    branch_forward as branch_fwd, branch_reverse as branch_rev,
};
use crate::const_util::const_map;

mod backports;
pub(crate) mod ext;
pub(crate) mod misc;

#[inline]
pub const fn branch_forward<T, E>(res: Result<T, E>) -> ControlFlow<E, T> {
    const_map! {
        match res;
        [Ok => Continue],
        [Err => Break],
    }
}

#[inline]
pub const fn branch_reverse<T, E>(res: Result<T, E>) -> ControlFlow<T, E> {
    const_map! {
        match res;
        [Ok => Break],
        [Err => Continue],
    }
}
