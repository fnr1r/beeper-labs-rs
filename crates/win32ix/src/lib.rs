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

#[cfg(feature = "reexport_windows")]
pub use windows as sys;

pub mod core;
pub mod storage;
pub mod system;
mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
