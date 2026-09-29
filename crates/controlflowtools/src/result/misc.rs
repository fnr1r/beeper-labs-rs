use core::ops::ControlFlow::{self, Break, Continue};

use easy_ext::ext;

#[ext(CfMiscResultExt)]
pub impl<B, C> ControlFlow<B, C> {
    #[inline]
    fn and_map_break<E>(
        self,
        f: impl Fn(B) -> ControlFlow<(), E>,
    ) -> ControlFlow<Result<(), E>, C> {
        match self {
            Continue(v) => Continue(v),
            Break(v) => Break(match f(v) {
                Break(()) => Ok(()),
                Continue(e) => Err(e),
            }),
        }
    }
}
