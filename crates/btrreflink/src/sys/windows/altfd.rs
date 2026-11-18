use std::fs::File;

pub use crate::stubs::{AsDirRef, AsFileRef, DirRef};

pub type FileRef<'a> = &'a mut File;

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
