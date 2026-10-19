use core::convert::Infallible;

type InfallibleCovariant<'a> = &'a Infallible;

pub type DirRef<'a> = InfallibleCovariant<'a>;
pub type FileRef<'a> = InfallibleCovariant<'a>;
