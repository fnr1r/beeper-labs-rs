use std::io::ErrorKind;

use crate::prelude::*;

#[inline]
pub(crate) fn unsupported_err() -> Error {
    ErrorKind::Unsupported.into()
}

#[inline]
pub(crate) fn unsupported() -> Result {
    Err(unsupported_err())
}
