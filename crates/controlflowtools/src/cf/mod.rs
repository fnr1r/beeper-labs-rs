use core::ops::ControlFlow::{self, Break, Continue};

pub use self::backports::into_value;

mod backports;
pub(crate) mod ext;

/// Converts any two-variant enum `this` into value of variant specified in the
/// 2nd argument, from the value of the variant passed as the 3rd arg with
/// callable passed as 4th arg.
#[macro_export]
macro_rules! cf_branch_join {
    ($this:expr, $variant_target:ident, $variant_source:ident, $f:expr) => {
        match $this {
            $variant_target(v) => v,
            $variant_source(v) => ($f)(v),
        }
    };
}

#[inline]
pub fn last_break<B, C>(this: ControlFlow<B, C>, f: impl FnOnce(B) -> C) -> C {
    cf_branch_join!(this, Continue, Break, f)
}

#[inline]
pub fn last_continue<B, C>(this: ControlFlow<B, C>, f: impl FnOnce(C) -> B) -> B {
    cf_branch_join!(this, Break, Continue, f)
}
