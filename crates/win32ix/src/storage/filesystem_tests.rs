use core::mem::transmute;
use std::sync::LazyLock;

use widestring::u16cstr;

use super::*;
use crate::{
    core::path::{StackPathAA, StackPathWA},
    system::library_loader::{current_exe_asa, current_exe_wsa},
};

macro_rules! cstr {
    ($text:expr) => {{
        const TEXT: &str = $text;
        const LEN: usize = TEXT.len();
        const BUF: &[u8] = TEXT.as_bytes();
        const LEN1: usize = LEN + 1;
        const STORAGE: [u8; LEN1] = {
            let mut buf = [0; _];
            let s = buf.split_at_mut(LEN).0;
            s.copy_from_slice(BUF);
            buf
        };
        unsafe { CStr::from_bytes_with_nul_unchecked(&STORAGE) }
    }};
}

const unsafe fn osstr_from_encoded_bytes_unchecked(s: &[u8]) -> &OsStr {
    unsafe { transmute::<&[u8], &OsStr>(s) }
}

const unsafe fn osstr_from_cstr(s: &CStr) -> &OsStr {
    unsafe { osstr_from_encoded_bytes_unchecked(s.to_bytes_with_nul()) }
}

const TEST_VOL: &str = "C:\\";
const TEST_VOL8: &CStr = cstr!(TEST_VOL);
const TEST_VOL8O: &OsStr = unsafe { osstr_from_cstr(TEST_VOL8) };
const TEST_VOL16: &U16CStr = u16cstr!(TEST_VOL);

static CURRENT_EXE_BUF_W: LazyLock<StackPathWA> = LazyLock::new(|| current_exe_wsa().unwrap());
static CURRENT_EXE_BUF_A: LazyLock<StackPathAA> = LazyLock::new(|| current_exe_asa().unwrap());

fn current_exe_w() -> &'static U16CStr {
    U16CStr::from_slice_truncate(&*CURRENT_EXE_BUF_W).unwrap()
}
fn current_exe_a() -> &'static CStr {
    CStr::from_bytes_until_nul(&*CURRENT_EXE_BUF_A).unwrap()
}
fn current_exe_aa() -> &'static OsStr {
    unsafe { osstr_from_cstr(current_exe_a()) }
}

#[test]
fn test_get_volume_path_name_w() {
    get_volume_path_name_w(current_exe_w()).unwrap();
}

#[test]
fn test_get_volume_path_name_a() {
    get_volume_path_name_a(current_exe_a()).unwrap();
}

#[test]
fn test_get_volume_path_name_aa() {
    get_volume_path_name_aa(current_exe_aa()).unwrap();
}

#[test]
fn test_get_volume_name_for_volume_mount_point_w() {
    get_volume_name_for_volume_mount_point_w(TEST_VOL16).unwrap();
}

#[test]
fn test_get_volume_name_for_volume_mount_point_a() {
    get_volume_name_for_volume_mount_point_a(TEST_VOL8).unwrap();
}

#[test]
fn test_get_volume_name_for_volume_mount_point_aa() {
    get_volume_name_for_volume_mount_point_aa(TEST_VOL8O).unwrap();
}

#[test]
fn test_get_volume_flags_w() {
    get_volume_flags_w(TEST_VOL16).unwrap();
}

#[test]
fn test_get_volume_flags_a() {
    get_volume_flags_a(TEST_VOL8).unwrap();
}

#[test]
fn test_get_volume_flags_aa() {
    get_volume_flags_aa(TEST_VOL8O).unwrap();
}
