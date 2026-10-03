//! `rustix` kinda thing but for windows and only one crate
//!
//! based on `https://github.com/cargo-bins/reflink-copy`
//!
//! ## NOTES
//!
//! - Rust MSRV
//! - `????` for dep `bytemuck` `1.4.1` (edition `2018`)
//! - `1.70` for dep `windows` `0.58.0`
//! - `1.71` for dep `bytemuck` `1.25.2`
//! - `1.76` for feature `ptr_from_ref`
//! - `1.82` for dep `windows` `0.62.2`
#![cfg(windows)]
#![allow(clippy::borrow_as_ptr)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(feature = "reexport_windows")]
pub use windows as sys;

pub mod core;
pub mod storage;
pub mod system;
mod utils;
