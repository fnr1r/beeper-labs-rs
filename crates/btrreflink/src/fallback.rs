use crate::prelude::*;

#[inline]
fn assert_len(copied: u64, len: Option<u64>) {
    if let Some(len) = len {
        debug_assert_eq!(copied, len);
    }
}

#[inline]
pub(crate) fn io_copy(src: FileRef, dst: FileRef, len: Option<u64>) -> Result {
    let res = std::io::copy(src, dst)?;
    assert_len(res, len);
    Ok(())
}

#[inline]
pub(crate) fn fs_copy(src: &Path, dst: &Path, len: Option<u64>) -> Result {
    let res = std::fs::copy(src, dst)?;
    assert_len(res, len);
    Ok(())
}
