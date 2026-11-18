use super::error::{Error, Result};

#[inline]
const fn mkresult(cond: bool) -> Result<(), ()> {
    if cond {
        Ok(())
    } else {
        Err(())
    }
}

#[inline]
fn mkresult2<E>(cond: bool, err_fn: impl FnOnce(()) -> E) -> Result<(), E> {
    mkresult(cond).map_err(err_fn)
}

#[inline]
fn xc2<E>(cond: bool, err_fn: impl FnOnce() -> E) -> Result<(), E> {
    mkresult2(cond, |()| err_fn())
}

#[inline]
fn ok_if(cond: bool) -> Result {
    xc2(cond, Error::from_thread)
}

#[inline]
pub(crate) fn err_if_zero(ret: u32) -> Result {
    ok_if(ret != 0)
}
