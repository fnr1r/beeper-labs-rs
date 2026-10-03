use core::ops::ControlFlow::{self, Break, Continue};
use std::io::ErrorKind;

use controlflowtools::result::branch_rev;

use crate::prelude::*;

#[inline]
fn maybe_unsupported<T>(res: Result<T>) -> ControlFlow<Result<T>, Error> {
    use ErrorKind as E;
    let err = branch_rev(res).map_break(Ok)?;
    Break(Err(match err.kind() {
        E::NotFound | E::PermissionDenied | E::AlreadyExists => err,
        _ => return Continue(err),
    }))
}

#[inline]
pub(super) fn implwrap<T>(res: Result<T>) -> ControlFlow<Result<T>> {
    maybe_unsupported(res).map_continue(drop)
}
