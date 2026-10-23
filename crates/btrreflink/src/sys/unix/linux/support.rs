use rustix::{fs::ioctl_ficlone, io::Errno, pipe::pipe};
use thiserror::Error;

type Error = IsReflinkSupportedError;

#[derive(Debug, Clone, Copy, Error)]
enum IsReflinkSupportedError {
    #[error("pipe() failed: {_0}")]
    PipeError(Errno),
    #[error("FICLONE between pipes succeeded???")]
    FicloneSuccess,
    #[error("unexpected FICLONE error: {_0}")]
    FicloneUnexpectedError(Errno),
}

#[inline]
fn is_reflink_supported_fallible() -> Result<bool, Error> {
    let (tx, rx) = pipe().map_err(Error::PipeError)?;
    Err(match ioctl_ficlone(tx, rx) {
        Err(Errno::INVAL) => return Ok(true),
        Err(Errno::NOTTY) => return Ok(false),
        Ok(()) => Error::FicloneSuccess,
        Err(e) => Error::FicloneUnexpectedError(e),
    })
}

/// Check if reflinks are supported
///
/// # Implementation note
///
/// Linux doesn't really have a separate syscall for checking if an `ioctl`
/// exists, but we can do an `ioctl` with invalid arguments. If we get an
/// `ENOTTY`, the syscall probably doesn't exist, if we get another error, it
/// does exist.
///
/// <https://www.man7.org/linux/man-pages/man2/ioctl.2.html>
/// <https://www.man7.org/linux/man-pages/man2/FICLONE.2const.html>
#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn is_reflink_supported() -> bool {
    is_reflink_supported_fallible()
        .inspect_err(|_e| {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                ?_e,
                "couldn't properly check for reflink support. falling back to false"
            );
        })
        .unwrap_or(false)
}
