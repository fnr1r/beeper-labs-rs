use core::{
    ops::ControlFlow::{self, Break, Continue},
    ptr::read,
};

use either::Either::{self, Left, Right};

use crate::const_util::man;

#[inline]
pub const fn continue_ok_or_break<T, E, B>(
    this: ControlFlow<B, Result<T, E>>,
) -> Result<Either<T, B>, E> {
    Ok(match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Continue(Ok(ref res)) => Left(unsafe { read(res) }),
        // SAFETY: the original data is ManuallyDrop
        &Continue(Err(ref err)) => return Err(unsafe { read(err) }),
        // SAFETY: the original data is ManuallyDrop
        &Break(ref res) => Right(unsafe { read(res) }),
    })
}

#[inline]
pub const fn break_ok_or_continue<T, E, C>(
    this: ControlFlow<Result<T, E>, C>,
) -> Result<Either<T, C>, E> {
    Ok(match man!(this) {
        // SAFETY: the original data is ManuallyDrop
        &Continue(ref res) => Right(unsafe { read(res) }),
        // SAFETY: the original data is ManuallyDrop
        &Break(Ok(ref res)) => Left(unsafe { read(res) }),
        // SAFETY: the original data is ManuallyDrop
        &Break(Err(ref err)) => return Err(unsafe { read(err) }),
    })
}

/// NOTE: for drop safety - `L` is [`Copy`] and can't impl [`Drop`]
#[inline]
pub(super) const fn either_cright<L: Copy, R>(this: Either<L, R>) -> Option<R> {
    match man!(this) {
        &Left(_) => None,
        // SAFETY: the original data is ManuallyDrop
        &Right(ref res) => Some(unsafe { read(res) }),
    }
}

#[inline]
pub const fn cresb_value<E, B>(this: ControlFlow<B, Result<(), E>>) -> Result<Option<B>, E> {
    match man!(continue_ok_or_break(this)) {
        // SAFETY: the original data is ManuallyDrop
        &Ok(ref res) => Ok(either_cright(unsafe { read(res) })),
        // SAFETY: the original data is ManuallyDrop
        &Err(ref err) => Err(unsafe { read(err) }),
    }
}

#[inline]
pub const fn cresc_value<E, C>(this: ControlFlow<Result<(), E>, C>) -> Result<Option<C>, E> {
    match man!(break_ok_or_continue(this)) {
        // SAFETY: the original data is ManuallyDrop
        &Ok(ref res) => Ok(either_cright(unsafe { read(res) })),
        // SAFETY: the original data is ManuallyDrop
        &Err(ref err) => Err(unsafe { read(err) }),
    }
}
