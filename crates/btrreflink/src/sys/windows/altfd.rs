use core::convert::Infallible;
use std::fs::File;

pub use crate::stubs::DirRef;

pub type FileRef<'a> = &'a mut File;

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
impl AsFileRef for File {
    fn as_file(&mut self) -> FileRef<'_> {
        self
    }
}
impl AsFileRef for &mut File {
    fn as_file(&mut self) -> FileRef<'_> {
        self
    }
}
