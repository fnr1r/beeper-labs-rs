#![allow(dead_code)]
#![allow(unreachable_pub)]
#![allow(clippy::absolute_paths)]

#[allow(unused_imports)]
pub use self::{funcs::*, traits::*, types::*};

mod error;
mod funcs;
mod infallible;
mod traits;
mod types;
