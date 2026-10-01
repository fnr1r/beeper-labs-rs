use std::{fs::File, os::fd::BorrowedFd};

pub type DirRef<'a> = BorrowedFd<'a>;
pub type FileRef<'a> = &'a mut File;

pub trait AsDirRef {
    fn as_dir(&mut self) -> BorrowedFd<'_>;
}
pub trait AsFileRef {
    fn as_file(&mut self) -> FileRef<'_>;
}

impl AsDirRef for BorrowedFd<'_> {
    fn as_dir(&mut self) -> BorrowedFd<'_> {
        *self
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
