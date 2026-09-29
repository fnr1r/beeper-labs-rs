use core::ops::ControlFlow;

pub use self::workarounds::{break_ok_or_continue, continue_ok_or_break, cresb_value, cresc_value};

mod workarounds;

#[inline]
pub fn resc_value<E, C, F: From<E>>(this: ControlFlow<Result<(), E>, C>) -> Result<Option<C>, F> {
    cresc_value(this).map_err(F::from)
}

#[inline]
pub fn resb_value<E, B, F: From<E>>(this: ControlFlow<B, Result<(), E>>) -> Result<Option<B>, F> {
    cresb_value(this).map_err(F::from)
}
