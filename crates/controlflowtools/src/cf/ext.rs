use core::ops::ControlFlow;

use easy_ext::ext;

#[ext(CfIntoValueExt)]
pub impl<T> ControlFlow<T, T> {
    #[inline]
    fn into_value(self) -> T {
        super::into_value(self)
    }
}

#[ext(CfBranchExt)]
pub impl<B, C> ControlFlow<B, C> {
    #[inline]
    fn b_to_c(self, f: impl FnOnce(B) -> C) -> C {
        super::last_break(self, f)
    }
    #[inline]
    fn c_to_b(self, f: impl FnOnce(C) -> B) -> B {
        super::last_continue(self, f)
    }
}

#[ext(CfWithValueExt)]
pub impl<B, C> ControlFlow<B, C> {
    #[inline]
    fn with_continue<T>(self, v: T) -> ControlFlow<B, T> {
        self.map_continue(|_| v)
    }
    #[inline]
    fn with_break<T>(self, v: T) -> ControlFlow<T, C> {
        self.map_break(|_| v)
    }
}
