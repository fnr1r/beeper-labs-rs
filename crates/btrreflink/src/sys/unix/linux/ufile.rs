use std::{ffi::OsStr, fs::File, os::fd::BorrowedFd};

use easy_ext::ext;
use rustix::{
    fs::{Mode, OFlags, openat},
    io::Result,
};

#[derive(Debug, Clone)]
pub(crate) struct OpenOptions {
    pub(crate) oflags: OFlags,
    pub(crate) _custom_flags: i32,
    pub(crate) mode: Mode,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            oflags: OFlags::empty(),
            _custom_flags: 0,
            mode: Mode::from_bits_truncate(0o666),
        }
    }
}

impl OpenOptions {
    pub(crate) fn read(&mut self, read: bool) -> &mut Self {
        self.oflags.set(OFlags::RDONLY, read);
        self
    }
    pub(crate) fn write(&mut self, write: bool) -> &mut Self {
        self.oflags.set(OFlags::WRONLY, write);
        self
    }
    pub(crate) fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.oflags.set(OFlags::CREATE | OFlags::EXCL, create_new);
        self
    }
    pub(crate) fn openat(&self, dirfd: BorrowedFd<'_>, path: &OsStr) -> Result<File> {
        openat(dirfd, path, self.oflags, self.mode).map(File::from)
    }
}

#[ext(UnixFileExt)]
pub impl File {
    fn uoptions() -> OpenOptions {
        OpenOptions::default()
    }
    fn read_at(dirfd: BorrowedFd<'_>, path: &OsStr) -> Result<File> {
        Self::uoptions().read(true).openat(dirfd, path)
    }
    fn create_new_at(dirfd: BorrowedFd<'_>, path: &OsStr) -> Result<File> {
        Self::uoptions()
            .write(true)
            .create_new(true)
            .openat(dirfd, path)
    }
}
