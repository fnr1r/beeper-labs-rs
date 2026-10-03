use core::{
    alloc::Layout,
    ffi::{CStr, c_void},
    mem::transmute_copy,
    ops::ControlFlow::{self, Break, Continue},
    ptr::NonNull,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};
use std::io::ErrorKind;

use controlflowtools::exts::{CfBranchExt, OptionExt};
use libc::{RTLD_DEFAULT, dlsym};

use crate::prelude::*;

const STATUS_UNKNOWN: u8 = 0;
const STATUS_UNSUPPORTED: u8 = 1;
const STATUS_SUPPORTED: u8 = 2;

static CLONEFILE_SUPPORTED: AtomicU8 = AtomicU8::new(0);

type Fun = NonNull<c_void>;
type SampleFn = unsafe extern "C" fn();

fn get_cached_ptr(storage: &AtomicUsize) -> ControlFlow<Fun> {
    NonNull::new(storage.load(Ordering::Relaxed) as _).or_cont(())
}

fn get_cached_support() -> ControlFlow<Error, bool> {
    Continue(match CLONEFILE_SUPPORTED.load(Ordering::Relaxed) {
        STATUS_UNKNOWN => true,
        STATUS_UNSUPPORTED => return Break(ErrorKind::Unsupported.into()),
        STATUS_SUPPORTED => false,
        _ => unreachable!(),
    })
}

unsafe fn cast_fn_pointer<T>(p: NonNull<c_void>) -> T {
    assert_eq!(Layout::new::<T>(), Layout::new::<SampleFn>());
    unsafe { transmute_copy(&p) }
}

unsafe fn dlsymd(symbol: &CStr) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe { dlsym(RTLD_DEFAULT, symbol.as_ptr()) })
}

fn get_dlsym_ptr(symbol: &CStr, storage: &AtomicUsize, store_support: bool) -> Result<Fun> {
    let (res, support) = if let Some(ptr) = unsafe { dlsymd(symbol) } {
        storage.store(ptr.addr().get(), Ordering::Relaxed);
        (Ok(ptr), STATUS_SUPPORTED)
    } else {
        (Err(Error::last_os_error()), STATUS_UNSUPPORTED)
    };
    store_support.then(|| CLONEFILE_SUPPORTED.store(support, Ordering::Relaxed));
    res
}

fn funptr_inner(storage: &AtomicUsize) -> ControlFlow<Result<Fun>, bool> {
    get_cached_ptr(storage).map_break(Ok)?;
    get_cached_support().map_break(Err)
}

pub(super) fn funptr<T>(symbol: &CStr, storage: &AtomicUsize) -> Result<T> {
    let cast_fn = |ptr| unsafe { cast_fn_pointer(ptr) };
    let f = |store_support| get_dlsym_ptr(symbol, storage, store_support);
    funptr_inner(storage).c_to_b(f).map(cast_fn)
}
