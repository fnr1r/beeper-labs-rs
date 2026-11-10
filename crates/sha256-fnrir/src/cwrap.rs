use cfg_if::cfg_if;
use sha2::{Digest, Sha256};

use crate::Bytes;

pub(crate) fn rust_sha256(data: &[u8]) -> Bytes {
    Sha256::digest(data).into()
}

#[cfg(feature = "backend_ring")]
pub(crate) fn ring_sha256(data: &[u8]) -> Bytes {
    use ring::digest::{SHA256, digest};
    digest(&SHA256, data).as_ref()[..32].try_into().unwrap()
}

cfg_if! {
    if #[cfg(all(feature = "backend_auto", not(debug_assertions)))] {
        pub(crate) use self::rust_sha256 as sha256;
    } else if #[cfg(feature = "backend_ring")] {
        pub(crate) use self::ring_sha256 as sha256;
    } else {
        pub(crate) use self::rust_sha256 as sha256;
    }
}
