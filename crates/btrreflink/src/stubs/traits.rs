use core::convert::Infallible;

use super::infallible::IoInfallible;
use crate::sys::{DirRef, FileRef};

pub trait AsDirRef {
    fn as_dir(&mut self) -> DirRef<'_>;
}
pub trait AsFileRef {
    fn as_file(&mut self) -> FileRef<'_>;
}

impl AsDirRef for Infallible {
    fn as_dir(&mut self) -> DirRef<'_> {
        match *self {}
    }
}
impl AsFileRef for Infallible {
    fn as_file(&mut self) -> FileRef<'_> {
        match *self {}
    }
}

impl AsDirRef for IoInfallible {
    fn as_dir(&mut self) -> DirRef<'_> {
        match self.0 {}
    }
}
impl AsFileRef for IoInfallible {
    fn as_file(&mut self) -> FileRef<'_> {
        match self.0 {}
    }
}
