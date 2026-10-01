use std::os::windows::io::AsRawHandle;

use win32ix::{
    core::Result,
    system::{
        ioctl::{FsctlGetIntegrityInformation, FsctlSetIntegrityInformation},
        ops::{fsctl_get_integrity_information, fsctl_set_integrity_information},
    },
};

use super::refs::ClusterSize;

pub(super) fn integrity_information_cvt(
    info: FsctlGetIntegrityInformation,
) -> FsctlSetIntegrityInformation {
    FsctlSetIntegrityInformation {
        ChecksumAlgorithm: info.ChecksumAlgorithm,
        Reserved: info.Reserved,
        Flags: info.Flags,
    }
}

pub(super) fn get_checked_cluster_size(
    src: &impl AsRawHandle,
    dst: &impl AsRawHandle,
) -> Result<Option<ClusterSize>> {
    let src_integrity_info = fsctl_get_integrity_information(src)?;
    let src_cluster_size = src_integrity_info.ClusterSizeInBytes;
    if src_cluster_size == 0 {
        return Ok(None);
    }
    let cluster_size = ClusterSize::try_from(src_cluster_size)?;
    // Copy over integrity information. Not sure if this is required.
    let dest_integrity_info = integrity_information_cvt(src_integrity_info);
    // ignore the error if it fails, the clone will still work
    if let Err(_e) = fsctl_set_integrity_information(dst, &dest_integrity_info) {
        #[cfg(feature = "tracing")]
        tracing::warn!(
            ?_e,
            "Failed to set integrity information (probably on DevDriver), but the clone still works"
        );
    }
    Ok(Some(cluster_size))
}
