use std::{
    ffi::{OsStr, OsString},
    os::windows::io::AsRawHandle,
};

use widestring::{U16CStr, U16CString};
use windows::Win32::Storage::FileSystem::{
    GetVolumeInformationA, GetVolumeInformationByHandleW, GetVolumeInformationW,
    GetVolumeNameForVolumeMountPointA, GetVolumeNameForVolumeMountPointW, GetVolumePathNameA,
    GetVolumePathNameW,
};
pub use windows::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_SPARSE_FILE, FILE_FLAGS_AND_ATTRIBUTES as FileFlagsAndAttributes,
};

use crate::{
    core::Result,
    foundation::MAX_PATH,
    utils::{
        as_handle, osstr_null_terminate, osstring_from_null_truncate, OsStrExt as _,
        U16CStrExt as _,
    },
};

pub fn get_volume_path_name_w(file_path: &U16CStr) -> Result<U16CString> {
    let path = file_path.as_internal();
    let mut buf = vec![0; MAX_PATH as _];
    unsafe { GetVolumePathNameW(path, &mut buf) }?;
    Ok(U16CString::from_vec_truncate(buf))
}

pub fn get_volume_path_name_a(file_path: &OsStr) -> Result<OsString> {
    let file_path = osstr_null_terminate(file_path)?;
    let path = file_path.as_internal();
    let mut buf = vec![0; MAX_PATH as _];
    unsafe { GetVolumePathNameA(path, &mut buf) }?;
    Ok(unsafe { osstring_from_null_truncate(buf, true) })
}

pub fn get_volume_name_for_volume_mount_point_w(volume_path: &U16CStr) -> Result<U16CString> {
    let path = volume_path.as_internal();
    let mut buf = vec![0; 50];
    unsafe { GetVolumeNameForVolumeMountPointW(path, &mut buf) }?;
    Ok(U16CString::from_vec_truncate(buf))
}

pub fn get_volume_name_for_volume_mount_point_a(volume_path: &OsStr) -> Result<OsString> {
    let volume_path = osstr_null_terminate(volume_path)?;
    let path = volume_path.as_internal();
    let mut buf = vec![0; 50];
    unsafe { GetVolumeNameForVolumeMountPointA(path, &mut buf) }?;
    Ok(unsafe { osstring_from_null_truncate(buf, true) })
}

pub fn get_volume_flags_w(volume_path: &U16CStr) -> Result<u32> {
    let path = volume_path.as_internal();
    let mut volume_flags = 0;
    let flags = Some(&raw mut volume_flags);
    unsafe { GetVolumeInformationW(path, None, None, None, flags, None) }?;
    Ok(volume_flags)
}

pub fn get_volume_flags_a(volume_path: &OsStr) -> Result<u32> {
    let volume_path = osstr_null_terminate(volume_path)?;
    let path = volume_path.as_internal();
    let mut volume_flags = 0;
    let flags = Some(&raw mut volume_flags);
    unsafe { GetVolumeInformationA(path, None, None, None, flags, None) }?;
    Ok(volume_flags)
}

pub fn get_volume_flags_by_handle(file: &impl AsRawHandle) -> Result<u32> {
    let file = as_handle(file);
    let mut volume_flags = 0;
    let flags = Some(&raw mut volume_flags);
    unsafe { GetVolumeInformationByHandleW(file, None, None, None, flags, None) }?;
    Ok(volume_flags)
}
