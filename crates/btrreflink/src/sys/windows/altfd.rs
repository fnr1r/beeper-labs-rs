use core::{convert::Infallible, marker::PhantomData};
use std::fs::File;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct NotExist<'a>(Infallible, PhantomData<&'a ()>);

pub type DirRef<'a> = NotExist<'a>;
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
