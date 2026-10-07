use cfg_if::cfg_if;

pub use crate::stubs::{SUPPORTS_REFLINKS, can_reflink_pp, is_reflink_supported};

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use self::windows::*;
    } else {
        pub use crate::stubs::*;
    }
}
