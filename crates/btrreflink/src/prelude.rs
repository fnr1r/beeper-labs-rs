pub(crate) use core::num::NonZeroU64;
pub(crate) use std::{ffi::OsStr, fs::File, io::Error, path::Path};

pub(crate) use crate::types::{AsDirRef, AsFileName, AsFilePath, AsFileRef, DirRef, FileRef};

pub(crate) type Result<T = (), E = Error> = core::result::Result<T, E>;
