use crate::prelude::*;
pub use crate::sys::{AsDirRef, AsFileRef, DirRef, FileRef};

pub trait AsFileName: AsRef<OsStr> {}
pub trait AsFilePath: AsRef<Path> {}

impl<T: AsRef<OsStr>> AsFileName for T {}
impl<T: AsRef<Path>> AsFilePath for T {}
