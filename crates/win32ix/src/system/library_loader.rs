use windows::Win32::{
    Foundation::HMODULE as HModule,
    System::LibraryLoader::{GetModuleFileNameA, GetModuleFileNameW},
};

use crate::core::{
    error_ext::err_if_zero,
    path::{try_stalloc, StackPathA, StackPathAA, StackPathW, StackPathWA},
    Result,
};

#[inline]
pub fn get_module_file_name_with_w(module: Option<HModule>, buf: &mut [u16]) -> Result {
    err_if_zero(unsafe { GetModuleFileNameW(module, buf) })
}

#[inline]
pub fn get_module_file_name_with_a(module: Option<HModule>, buf: &mut [u8]) -> Result {
    err_if_zero(unsafe { GetModuleFileNameA(module, buf) })
}

#[inline]
pub fn current_exe_with_w(buf: &mut [u16]) -> Result {
    get_module_file_name_with_w(None, buf)
}

#[inline]
pub fn current_exe_with_a(buf: &mut [u8]) -> Result {
    get_module_file_name_with_a(None, buf)
}

#[inline]
pub fn current_exe_wsn<const N: usize>() -> Result<StackPathW<N>> {
    try_stalloc!(current_exe_with_w)
}

#[inline]
pub fn current_exe_asn<const N: usize>() -> Result<StackPathA<N>> {
    try_stalloc!(current_exe_with_a)
}

#[inline]
pub fn current_exe_wsa() -> Result<StackPathWA> {
    current_exe_wsn()
}

#[inline]
pub fn current_exe_asa() -> Result<StackPathAA> {
    current_exe_asn()
}
