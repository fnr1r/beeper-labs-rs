use core::num::{NonZeroU32, NonZeroU64};
use std::io::{Error as IoError, ErrorKind};

use derive_more::Constructor;
use thiserror::Error;
use win32ix::{
    core::WinError,
    sys::{Win32::Foundation::ERROR_INVALID_DATA, core::HRESULT},
};

const KIB: u32 = super::KIB as _;

const CLUSTER_SIZE_ERR: &str =
    "Cluster size of source must either be 4K or 64K (restricted by ReFS)";

pub(super) type ClusterSize = ReFSClusterSize;
pub(super) type ClusterSizeError = ReFSClusterSizeError;
type Result<T, E = ClusterSizeError> = core::result::Result<T, E>;

#[derive(Debug, Error, Constructor)]
#[error("{CLUSTER_SIZE_ERR}\ngot: {0}")]
pub(super) struct ReFSClusterSizeError(u32);

impl From<ClusterSizeError> for IoError {
    fn from(value: ClusterSizeError) -> Self {
        IoError::new(ErrorKind::InvalidData, value)
    }
}

impl From<ClusterSizeError> for WinError {
    fn from(value: ClusterSizeError) -> Self {
        let msg = format!("{}", value).into_boxed_str();
        WinError::new(HRESULT::from(ERROR_INVALID_DATA), msg)
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub(super) enum ReFSClusterSize {
    C4K = 4 * KIB,
    #[default]
    C64K = 64 * KIB,
}

impl ClusterSize {
    const C4KN: u32 = Self::C4K as _;
    const C64KN: u32 = Self::C64K as _;
    #[allow(dead_code)]
    pub(super) const MAX: Self = Self::C64K;
}

pub(super) const fn cluster_size_from_repr(cluster_size: u32) -> Result<ClusterSize> {
    use ClusterSize as E;
    Ok(match cluster_size {
        E::C4KN => E::C4K,
        E::C64KN => E::C64K,
        _ => return Err(ClusterSizeError::new(cluster_size)),
    })
}

impl TryFrom<u32> for ClusterSize {
    type Error = ClusterSizeError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        cluster_size_from_repr(value)
    }
}

impl TryFrom<Option<NonZeroU64>> for ClusterSize {
    type Error = ClusterSizeError;
    fn try_from(value: Option<NonZeroU64>) -> Result<Self, Self::Error> {
        let v = value.map(NonZeroU64::get).unwrap_or_default();
        let cluster_size = v.try_into().map_err(|_| todo!())?;
        cluster_size_from_repr(cluster_size)
    }
}

const _: () = assert!(cluster_size_from_repr(0).is_err());

impl From<ClusterSize> for NonZeroU32 {
    fn from(value: ClusterSize) -> Self {
        unsafe { Self::new_unchecked(value as _) }
    }
}

impl From<ClusterSize> for NonZeroU64 {
    fn from(value: ClusterSize) -> Self {
        NonZeroU32::from(value).into()
    }
}

impl From<ClusterSize> for u32 {
    fn from(value: ClusterSize) -> Self {
        value as _
    }
}

impl From<ClusterSize> for u64 {
    fn from(value: ClusterSize) -> Self {
        value as _
    }
}
