use core::ops::ControlFlow;

use easy_ext::ext;

#[ext(ResultExt)]
pub impl<T, E> Result<T, E> {
    #[inline]
    fn branch_fwd(self) -> ControlFlow<E, T> {
        super::branch_forward(self)
    }
    #[inline]
    fn branch_rev(self) -> ControlFlow<T, E> {
        super::branch_reverse(self)
    }
}

#[ext(CfResultAsContinueExt)]
pub impl<T, E, C> ControlFlow<Result<T, E>, C> {
    #[inline]
    fn map_err<F>(self, op: impl FnOnce(E) -> F) -> ControlFlow<Result<T, F>, C> {
        self.map_break(|e| e.map_err(op))
    }
}

/// NOTE: backported from Rust `1.96`
#[ext(CfToResultBackportsExt)]
pub impl<B, C> ControlFlow<B, C> {
    #[inline]
    fn break_ok(self) -> Result<B, C> {
        super::break_ok(self)
    }
    #[inline]
    fn continue_ok(self) -> Result<C, B> {
        super::continue_ok(self)
    }
}
