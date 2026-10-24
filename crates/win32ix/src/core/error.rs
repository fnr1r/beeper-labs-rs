pub use windows::core::{Error, HRESULT as HResult};

pub use self::{Error as WinError, Result as WinResult};

#[allow(clippy::absolute_paths)]
pub type Result<T = (), E = Error> = core::result::Result<T, E>;
