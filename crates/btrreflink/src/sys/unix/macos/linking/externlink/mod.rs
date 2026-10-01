use super::prototypes::{self, Clonefile, ClonefileAt, FClonefileAt};
use crate::prelude::*;

mod cextern;

pub(in crate::sys::unix::macos) fn get_clonefile() -> Result<Clonefile> {
    Ok(self::cextern::clonefile)
}

pub(in crate::sys::unix::macos) fn get_clonefileat() -> Result<ClonefileAt> {
    Ok(self::cextern::clonefileat)
}

pub(in crate::sys::unix::macos) fn get_fclonefileat() -> Result<FClonefileAt> {
    Ok(self::cextern::fclonefileat)
}
