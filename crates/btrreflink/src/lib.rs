#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::std_instead_of_alloc)]
#![allow(clippy::undocumented_unsafe_blocks)]

use cfg_if::cfg_if;

pub use crate::{
    only::{can_reflink_pp, is_reflink_supported},
    sys::SUPPORTS_REFLINKS,
};

cfg_if! {
    if #[cfg(feature = "copy_fallback")] {
        pub use crate::or_copy::{reflink_or_copy_ff as reflink_ff, reflink_or_copy_pp as reflink_pp};
    } else {
        pub use crate::only::{reflink_ff, reflink_pp};
    }
}

pub mod checking;
mod fallback;
pub mod maybe;
mod maybewrap;
pub mod only;
pub mod or_copy;
mod prelude;
mod stubs;
pub mod sys;
pub mod types;
