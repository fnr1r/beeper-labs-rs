use cfg_if::cfg_if;

pub use self::fd::{AsDirRef, AsFileRef, DirRef, FileRef};
pub use crate::stubs::{SUPPORTS_REFLINKS, can_reflink_pp, is_reflink_supported};

mod errno;
mod fd;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use self::macos::*;
    } else {
        pub use crate::stubs::*;
    }
}
