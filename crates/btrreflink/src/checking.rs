pub use crate::sys::{can_reflink_pp, is_reflink_supported};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflinkSupported {
    Unknown,
    Supported,
    Unsupported,
}
