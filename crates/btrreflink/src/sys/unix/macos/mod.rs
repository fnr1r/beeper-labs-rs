//! can't reflink from fds on macos

use std::{ffi::OsStr, os::fd::AsFd};

use cfg_if::cfg_if;

use self::{
    cfuncwrap::{clonefile, clonefileat, fclonefileat},
    oscstr::cstr,
};
use crate::prelude::*;
pub use crate::stubs::{kernel_copy, reflink_block};

mod cfuncwrap;
mod linking;
mod oscstr;

cfg_if! {
    if #[cfg(feature = "macos_hax")] {
        mod fcntls;
        mod hacky;
        pub use self::hacky::reflink;
    } else {
        pub use crate::stubs::reflink;
    }
}

pub fn reflink_fd(srcfd: FileRef, dstfd: DirRef, dst: &OsStr) -> Result<()> {
    let srcfd = srcfd.as_fd();
    let dstfd = dstfd.as_fd();
    let dst = cstr(dst)?;
    fclonefileat(srcfd, dstfd, dst.as_ref())
}

pub fn reflink_dd(srcfd: DirRef, src: &OsStr, dstfd: DirRef, dst: &OsStr) -> Result<()> {
    let srcfd = srcfd.as_fd();
    let src = cstr(src)?;
    let dstfd = dstfd.as_fd();
    let dst = cstr(dst)?;
    clonefileat(srcfd, src.as_ref(), dstfd, dst.as_ref())
}

pub fn reflink_pp(src: &Path, dst: &Path) -> Result<()> {
    let src = cstr(src.as_os_str())?;
    let dst = cstr(dst.as_os_str())?;
    clonefile(src.as_ref(), dst.as_ref())
}
