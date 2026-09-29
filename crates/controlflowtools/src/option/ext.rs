use core::ops::ControlFlow;

use easy_ext::ext;

#[ext(OptionExt)]
pub impl<T> Option<T> {
    #[inline]
    fn branch_fwd(self) -> ControlFlow<(), T> {
        super::branch_forward(self)
    }
    #[inline]
    fn branch_rev(self) -> ControlFlow<T, ()> {
        super::branch_reverse(self)
    }
    #[inline]
    fn or_break<B>(self, b: B) -> ControlFlow<B, T> {
        super::or_break(self, b)
    }
    #[inline]
    fn or_cont<C>(self, c: C) -> ControlFlow<T, C> {
        super::or_cont(self, c)
    }
}
