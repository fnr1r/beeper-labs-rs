use std::{
    os::windows::{ffi::OsStrExt as _, io::AsRawHandle},
    path::Path,
};

pub use windows::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_SPARSE_FILE as FileAttributeSparseFile,
    FILE_FLAGS_AND_ATTRIBUTES as FileFlagsAndAttributes,
};
use windows::{
    core::{Result as WinResult, PCWSTR},
    Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::{
            GetVolumeInformationByHandleW, GetVolumeInformationW,
            GetVolumeNameForVolumeMountPointW, GetVolumePathNameW,
        },
        System::SystemServices::FILE_SUPPORTS_BLOCK_REFCOUNTING,
    },
};

use crate::utils::as_handle;

/// A wrapper function for
/// [GetVolumePathNameW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumepathnamew)
/// that retrieves the volume mount point where the specified path is mounted.
fn get_volume_path(path: impl AsRef<Path>) -> WinResult<Vec<u16>> {
    let path_wide: Vec<u16> = path
        .as_ref()
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();
    let mut volume_name_buffer = vec![0u16; MAX_PATH as usize];

    unsafe { GetVolumePathNameW(PCWSTR(path_wide.as_ptr()), volume_name_buffer.as_mut()) }?;

    if let Some(pos) = volume_name_buffer.iter().position(|&c| c == 0) {
        volume_name_buffer.truncate(pos);
    }

    Ok(volume_name_buffer)
}

/// A wrapper function for
/// [GetVolumeNameForVolumeMountPointW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumenameforvolumemountpointw)
/// that retrieves a volume GUID path for the volume that is associated with the specified volume
/// mount point (drive letter, volume GUID path, or mounted folder).
fn get_volume_guid_path(volume_path_w: &[u16]) -> WinResult<Vec<u16>> {
    let mut volume_guid_path = vec![0u16; 50usize];
    unsafe {
        GetVolumeNameForVolumeMountPointW(PCWSTR(volume_path_w.as_ptr()), volume_guid_path.as_mut())
    }?;

    if let Some(pos) = volume_guid_path.iter().position(|&c| c == 0) {
        volume_guid_path.truncate(pos);
    }

    Ok(volume_guid_path)
}

/// A wrapper function for
/// [GetVolumeInformationW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumeinformationw)
/// that returns `FileSystemFlags`.
fn get_volume_flags(volume_path_w: &[u16]) -> WinResult<u32> {
    let mut file_system_flags = 0u32;

    unsafe {
        GetVolumeInformationW(
            PCWSTR(volume_path_w.as_ptr()),
            None,
            None,
            None,
            Some(&mut file_system_flags),
            None,
        )
    }?;

    Ok(file_system_flags)
}

/// Checks whether reflink is supported on the filesystem for the specified source and target paths.
///
/// This function verifies that both paths are on the same volume and that the filesystem supports
/// reflink.
pub fn check_reflink_support(from: impl AsRef<Path>, to: impl AsRef<Path>) -> WinResult<bool> {
    let from_volume = get_volume_path(from)?;
    let to_volume = get_volume_path(to)?;

    let from_guid = get_volume_guid_path(&from_volume)?;
    let to_guid = get_volume_guid_path(&to_volume)?;
    if from_guid != to_guid {
        // The source and destination files must be on the same volume
        return Ok(false);
    }

    let volume_flags = get_volume_flags(&from_volume)?;
    Ok(volume_flags & FILE_SUPPORTS_BLOCK_REFCOUNTING != 0)
}

pub fn is_block_cloning_supported(file: &impl AsRawHandle) -> WinResult<bool> {
    let mut flags = 0u32;
    unsafe {
        GetVolumeInformationByHandleW(as_handle(file), None, None, None, Some(&mut flags), None)
    }?;
    Ok((flags & FILE_SUPPORTS_BLOCK_REFCOUNTING) != 0)
}
