use core::mem::{MaybeUninit, transmute};
use std::{
    os::{
        fd::{AsRawFd, BorrowedFd},
        unix::ffi::OsStrExt,
    },
    path::PathBuf,
};

use libc::{F_GETPATH, MAXPATHLEN, fcntl};

use crate::{prelude::*, utils::errno};

const BUF_SIZE: usize = MAXPATHLEN as usize;

type UBuf = [MaybeUninit<u8>; BUF_SIZE];
//type Buf = [u8; BUF_SIZE];
type UDat = [MaybeUninit<u8>];
type Dat = [u8];

#[inline]
unsafe fn assume_init_mut<T>(this: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: MaybeUninit is repr(transparent)
    unsafe { transmute(this) }
}

#[inline]
unsafe fn assume_partial_init_mut<T>(
    this: &mut [MaybeUninit<T>],
    len: usize,
) -> (&mut [T], &mut [MaybeUninit<T>]) {
    let (a, b) = this.split_at_mut(len);
    // SAFETY: guaranteed by the caller
    (unsafe { assume_init_mut(a) }, b)
}

fn fcntl_getpath_buf<'a>(
    fd: BorrowedFd<'_>,
    buf: &'a mut UBuf,
) -> Result<(&'a mut Dat, &'a mut UDat)> {
    let fd = fd.as_raw_fd();
    let cmd = F_GETPATH;
    let res = unsafe { fcntl(fd, cmd, buf.as_mut_ptr()) };
    errno(res)?;
    Ok(unsafe { assume_partial_init_mut(buf, res as usize) })
}

fn fcntl_getpath_with<T>(
    fd: BorrowedFd<'_>,
    f: impl FnOnce((&mut Dat, &mut UDat)) -> T,
) -> Result<T> {
    let mut buf: UBuf = MaybeUninit::<[u8; BUF_SIZE]>::uninit().into();
    fcntl_getpath_buf(fd, &mut buf).map(f)
}

pub(in super::super) fn fcntl_getpath(fd: BorrowedFd<'_>) -> Result<PathBuf> {
    fcntl_getpath_with(fd, |(path, _)| PathBuf::from(OsStr::from_bytes(path)))
}
