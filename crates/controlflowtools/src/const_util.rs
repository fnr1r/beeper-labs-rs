use core::{mem::ManuallyDrop, ptr::from_ref};

#[cfg(any(feature = "option", feature = "result"))]
pub(crate) use crate::__const_map as const_map;
pub(crate) use crate::__man as man;

pub(crate) const fn man_drop_ref<T>(man: &ManuallyDrop<T>) -> &T {
    // SAFETY: repr(transparent)
    unsafe { &*from_ref(man).cast() }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __man {
    ($this:expr) => {
        $crate::const_util::man_drop_ref(&::core::mem::ManuallyDrop::new($this))
    };
}

#[cfg(any(feature = "option", feature = "result"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __const_map {
    (
        match $this:expr;
        $([
            $rule1_key_from:ident => $rule1_key_to:ident
        ]),* $(,)?
        $(;
            $(empty [
                $rule2_key_from:ident => $rule2_key_to:ident
            ]),* $(,)?
        )?
    ) => {
        match $crate::const_util::man!($this) {
            $(
                &$rule1_key_from(ref res) => {
                    // SAFETY: the original data is ManuallyDrop
                    let res = unsafe { ::core::ptr::read(res) };
                    $rule1_key_to(res)
                }
            ),*
            $($(
                &$rule2_key_from => $rule2_key_to(())
            ),*)?
        }
    };
}
