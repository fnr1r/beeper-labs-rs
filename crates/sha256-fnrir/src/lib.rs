#[cfg(feature = "binrw")]
pub use binrw;

pub use self::digest::Sha256Digest;

mod cwrap;
pub mod digest;

type Bytes = [u8; 32];
type HexBytes = [u8; 64];

#[cfg(feature = "binrw")]
#[macro_export]
macro_rules! brw_endian_none {
    ($name:ident) => {
        impl $crate::binrw::meta::ReadEndian for $name {
            const ENDIAN: $crate::binrw::meta::EndianKind = $crate::binrw::meta::EndianKind::None;
        }

        impl $crate::binrw::meta::WriteEndian for $name {
            const ENDIAN: $crate::binrw::meta::EndianKind = $crate::binrw::meta::EndianKind::None;
        }
    };
}
