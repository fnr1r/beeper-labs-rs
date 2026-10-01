use core::sync::atomic::AtomicUsize;

use self::dynld::funptr;
use super::prototypes::{Clonefile, ClonefileAt, FClonefileAt};
use crate::prelude::*;

mod dynld;
#[cfg(feature = "macos_dynlink_eager")]
mod eager;

static CLONEFILE: AtomicUsize = AtomicUsize::new(0);
static CLONEFILE_AT: AtomicUsize = AtomicUsize::new(0);
static F_CLONEFILE_AT: AtomicUsize = AtomicUsize::new(0);

pub(in crate::sys::unix::macos) fn get_clonefile() -> Result<Clonefile> {
    funptr(c"clonefile", &CLONEFILE)
}

pub(in crate::sys::unix::macos) fn get_clonefileat() -> Result<ClonefileAt> {
    funptr(c"clonefileat", &CLONEFILE_AT)
}

pub(in crate::sys::unix::macos) fn get_fclonefileat() -> Result<FClonefileAt> {
    funptr(c"fclonefileat", &F_CLONEFILE_AT)
}
