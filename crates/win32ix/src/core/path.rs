use windows::Win32::Foundation::MAX_PATH as MAX_PATH_U32;

pub(crate) use crate::__try_stalloc as try_stalloc;

const MAX_PATH: usize = MAX_PATH_U32 as _;
const ALIGNED: usize = 2usize.pow(MAX_PATH.ilog2() + 1);

pub(crate) type StackPathW<const N: usize = MAX_PATH> = [u16; N];
pub(crate) type StackPathA<const N: usize = MAX_PATH> = [u8; N];
pub(crate) type StackPathWA<const N: usize = ALIGNED> = StackPathW<N>;
pub(crate) type StackPathAA<const N: usize = ALIGNED> = StackPathA<N>;

#[doc(hidden)]
#[macro_export]
macro_rules! __try_stalloc {
    ($f:expr) => {{
        let mut buf = [0; _];
        match ($f)(&mut buf) {
            Ok(()) => Ok(buf),
            Err(e) => Err(e),
        }
    }};
}
