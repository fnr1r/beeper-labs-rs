use core::convert::Infallible;
use std::io::{Read, Result, Write};

#[derive(Debug, Clone, Copy)]
pub struct IoInfallible(pub(super) Infallible);

impl Read for IoInfallible {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        match self.0 {}
    }
}

impl Write for IoInfallible {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        match self.0 {}
    }
    fn flush(&mut self) -> Result<()> {
        match self.0 {}
    }
}
