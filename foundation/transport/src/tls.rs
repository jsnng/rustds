use core::fmt::Debug;
#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(feature = "std")]
pub trait TlsHandshaker {
    type Connection;
    type HandshakeError: Debug;
    fn handshake<S: Read + Write>(&self, server_name: &str, stream: &mut S) -> Result<Self::Connection, Self::HandshakeError>;
}
