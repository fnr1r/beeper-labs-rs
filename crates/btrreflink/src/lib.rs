use cfg_if::cfg_if;

pub use crate::{
    only::{can_reflink_pp, is_reflink_supported},
    sys::SUPPORTS_REFLINKS,
};

cfg_if! {
    if #[cfg(feature = "copy_fallback")] {
        pub use crate::or_copy::{reflink_or_copy as reflink, reflink_or_copy_pp as reflink_pp};
    } else {
        pub use crate::only::{reflink, reflink_pp};
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
