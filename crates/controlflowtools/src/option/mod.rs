use core::ops::ControlFlow::{self, Break, Continue};

pub use self::{
    branch_forward as branch_fwd, branch_reverse as branch_rev, or_continue as or_cont,
    workarounds::{cbreak_value, ccontinue_value, cor_break, cor_continue},
};
use crate::const_util::const_map;

pub(crate) mod ext;
#[cfg(test)]
mod tests;
mod workarounds;

#[inline]
pub const fn branch_forward<T>(res: Option<T>) -> ControlFlow<(), T> {
    const_map! {
        match res;
        [Some => Continue];
        empty [None => Break],
    }
}

#[inline]
pub const fn branch_reverse<T>(res: Option<T>) -> ControlFlow<T, ()> {
    const_map! {
        match res;
        [Some => Break];
        empty [None => Continue],
    }
}

#[inline]
pub fn or_break<T, B>(res: Option<T>, b: B) -> ControlFlow<B, T> {
    branch_forward(res).map_break(|()| b)
}

#[inline]
pub fn or_continue<T, C>(res: Option<T>, c: C) -> ControlFlow<T, C> {
    branch_reverse(res).map_continue(|()| c)
}
