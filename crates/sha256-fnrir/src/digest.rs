use core::fmt::{Debug, Display, Formatter, LowerHex, Result as FmtResult, UpperHex};

use std::io::{Read, Result as IoResult};

use ascii::AsciiStr;
#[cfg(feature = "binrw")]
use binrw::binrw;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use const_hex::{FromHexError, const_decode_to_array, encode_to_slice, encode_to_slice_upper};
#[cfg(feature = "mmap")]
use memmap2::{Mmap, MmapAsRawDesc};

use crate::{Bytes, HexBytes, cwrap::sha256};

#[rustfmt::skip]
#[cfg_attr(feature = "binrw", binrw)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "const-default", derive(const_default::ConstDefault))]
#[cfg_attr(feature = "bytemuck", derive(Zeroable, Pod))]
#[repr(transparent)]
pub struct Sha256Digest(pub Bytes);

#[cfg(feature = "binrw")]
crate::brw_endian_none!(Sha256Digest);

impl Sha256Digest {
    pub const fn const_from_str(s: &str) -> Result<Self, FromHexError> {
        match const_decode_to_array(s.as_bytes()) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(e),
        }
    }
    pub fn csum(data: &[u8]) -> Self {
        Self(sha256(data))
    }
    #[cfg(feature = "mmap")]
    pub fn csum_mmap(file: impl MmapAsRawDesc) -> IoResult<Self> {
        #![expect(clippy::undocumented_unsafe_blocks)]
        let buf = unsafe { Mmap::map(file) }?;
        Ok(Self::csum(&buf))
    }
    pub fn csum_force_read(file: &mut impl Read) -> IoResult<Self> {
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        Ok(Self::csum(&buf))
    }
    fn as_hex_bytes(&self) -> HexBytes {
        let mut buf = [0; 64];
        encode_to_slice(self.0, &mut buf).unwrap();
        buf
    }
    fn as_hex_bytes_upper(&self) -> HexBytes {
        let mut buf = [0; 64];
        encode_to_slice_upper(self.0, &mut buf).unwrap();
        buf
    }
    #[inline]
    fn with_hex<T>(&self, f: impl FnOnce(&HexBytes) -> T) -> T {
        f(&self.as_hex_bytes())
    }
    #[inline]
    fn with_hex_upper<T>(&self, f: impl FnOnce(&HexBytes) -> T) -> T {
        f(&self.as_hex_bytes_upper())
    }
    #[inline]
    fn with_hex_ascii_str<T>(&self, f: impl FnOnce(&AsciiStr) -> T) -> T {
        self.with_hex(move |s| f(AsciiStr::from_ascii(&s).unwrap()))
    }
    #[inline]
    fn with_hex_ascii_str_upper<T>(&self, f: impl FnOnce(&AsciiStr) -> T) -> T {
        self.with_hex_upper(move |s| f(AsciiStr::from_ascii(&s).unwrap()))
    }
    #[inline]
    pub fn with_hex_str<T>(&self, f: impl FnOnce(&str) -> T) -> T {
        self.with_hex_ascii_str(move |s| f(s.as_str()))
    }
    #[inline]
    pub fn with_hex_str_upper<T>(&self, f: impl FnOnce(&str) -> T) -> T {
        self.with_hex_ascii_str_upper(move |s| f(s.as_str()))
    }
}

impl LowerHex for Sha256Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.with_hex_str(|s| f.write_str(s))
    }
}

impl UpperHex for Sha256Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.with_hex_str_upper(|s| f.write_str(s))
    }
}

impl Display for Sha256Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as LowerHex>::fmt(self, f)
    }
}

impl Debug for Sha256Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("sha256:")?;
        <Self as LowerHex>::fmt(self, f)
    }
}
