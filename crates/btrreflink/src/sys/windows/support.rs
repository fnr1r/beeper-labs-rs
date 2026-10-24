use win32ix::{
    core::{
        error::{Error, HResult, Result},
        widestring::{U16CString, error::ContainsNul},
    },
    foundation::ERROR_INVALID_PARAMETER,
    storage::filesystem::{
        get_volume_flags_w, get_volume_name_for_volume_mount_point_w, get_volume_path_name_w,
    },
    system::system_services::FILE_SUPPORTS_BLOCK_REFCOUNTING,
};

use crate::prelude::*;

fn err_invalid_param(message: impl AsRef<str>) -> Error {
    Error::new(HResult::from(ERROR_INVALID_PARAMETER), message)
}

const CONTAINS_NULL: &str = "passed a Path containing non-terminator null-bytes";

#[allow(clippy::needless_pass_by_value)]
fn u16winerr(error: ContainsNul<u16>) -> Error {
    let pos = error.nul_position();
    err_invalid_param(format!("{CONTAINS_NULL} at: {pos}"))
}

const fn flags_supports_block_refcounting(flags: u32) -> bool {
    flags & FILE_SUPPORTS_BLOCK_REFCOUNTING != 0
}

pub(super) fn check_reflink_support(src: &Path, dst: &Path) -> Result<bool> {
    let src = U16CString::from_os_str(src).map_err(u16winerr)?;
    let dst = U16CString::from_os_str(dst).map_err(u16winerr)?;
    let src_volume = get_volume_path_name_w(&src)?;
    let dst_volume = get_volume_path_name_w(&dst)?;
    let src_guid = get_volume_name_for_volume_mount_point_w(&src_volume)?;
    let dst_guid = get_volume_name_for_volume_mount_point_w(&dst_volume)?;
    if src_guid != dst_guid {
        // The source and destination files must be on the same volume
        return Ok(false);
    }
    get_volume_flags_w(&src_volume).map(flags_supports_block_refcounting)
}
