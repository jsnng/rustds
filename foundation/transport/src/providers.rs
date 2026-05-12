#[cfg(feature = "std")]
use crate::prelude::*;
#[cfg(feature = "std")]
use std::io::{Read, Write};
#[cfg(feature = "std")]
use std::net::TcpStream;

#[cfg(feature = "std")]
impl AsyncTransport for TcpStream {
    type Error = std::io::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Read::read(self, buf)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Write::write(self, buf)
    }

    fn set_read_timeout(
        &mut self,
        timeout: Option<core::time::Duration>,
    ) -> Result<(), Self::Error> {
        TcpStream::set_read_timeout(self, timeout)
    }

    fn set_write_timeout(
        &mut self,
        timeout: Option<core::time::Duration>,
    ) -> Result<(), Self::Error> {
        TcpStream::set_write_timeout(self, timeout)
    }
}
