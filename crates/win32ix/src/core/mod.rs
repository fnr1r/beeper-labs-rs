pub use widestring::{self, U16CStr, U16CString};

pub use self::{
    error::{Error, HResult, Result, WinError, WinResult},
    string::{PCStr, PCWStr, PStr, PWStr},
};

pub mod error;
pub mod string;
