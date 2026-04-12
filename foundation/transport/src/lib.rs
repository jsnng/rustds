#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod prelude;
#[cfg(feature = "tls")]
pub mod tls;
#[cfg(feature = "rustls")]
pub mod rustls;
pub mod providers;

use core::time::Duration;

/// `Transport` is a low-level I/O abstraction that handles
/// reading and writing raw byes. It is used to encapsulate
/// the TDS protocol logic (such as encoding/decoding messages,
/// state transitions etc.) on top of the transport layer.
pub trait Transport {
    type Error;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;
    fn set_read_timeout(&mut self, timeout: Option<Duration>) -> Result<(), Self::Error>;
    fn set_write_timeout(&mut self, timeout: Option<Duration>) -> Result<(), Self::Error>;
}

/// `Send` encodes a message struct `M` and uses `write` to transport it.
pub trait Sender<M, T: Transport> {
    type Error;
    fn send(&mut self, msg: M) -> Result<(), Self::Error>;
}

/// `Receiver` reads incoming TDS traffic into the session's internal buffer
/// and returns a zero-copy span `Output<'_>` that borrows directly from it.
pub trait Receiver<T: Transport> {
    type Error;
    type Output<'a>
    where
        Self: 'a;
    fn receive(&mut self) -> Result<Self::Output<'_>, Self::Error>;
}