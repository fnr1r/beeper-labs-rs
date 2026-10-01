use cfg_if::cfg_if;

pub use self::fd::{AsDirRef, AsFileRef, DirRef, FileRef};

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
