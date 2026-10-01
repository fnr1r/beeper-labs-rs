#![no_std]

use core::iter::{RepeatWith, Take, repeat_with};

pub type RepeatFn<T> = Take<RepeatWith<T>>;

pub fn repeat_fn<T, F: FnMut() -> T>(f: F, n: usize) -> RepeatFn<F> {
    repeat_with(f).take(n)
}

pub fn iter_retry<T>(f: impl FnMut() -> Option<T>, n: usize) -> Option<T> {
    repeat_fn(f, n).flatten().next()
}
