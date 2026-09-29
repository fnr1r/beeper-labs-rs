#![allow(clippy::missing_errors_doc, reason = "these are generic utils")]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::match_ref_pats, reason = "conflict with pattern_type_mismatch")]
#![allow(
    clippy::needless_borrowed_reference,
    reason = "conflict with const workaround"
)]
#![no_std]

#[cfg(test)]
extern crate alloc;

pub mod cf;
mod const_util;
#[cfg(feature = "either")]
pub mod either;
pub mod exts;
#[cfg(feature = "option")]
pub mod option;
#[cfg(feature = "result")]
pub mod result;
