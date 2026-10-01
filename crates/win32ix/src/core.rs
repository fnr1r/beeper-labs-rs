pub use self::{WinError as Error, WinResult as Result};

pub type WinError = windows::core::Error;
pub type WinResult<T = (), E = Error> = core::result::Result<T, E>;
