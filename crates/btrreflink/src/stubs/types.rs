use core::convert::Infallible;

use super::infallible::IoInfallible;

pub(super) type Dir = Infallible;
pub(super) type File = IoInfallible;
pub type DirRef<'a> = &'a mut Dir;
pub type FileRef<'a> = &'a mut File;
