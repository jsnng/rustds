use core::fmt::Debug;
use crate::AsyncTransport;

#[cfg(feature = "std")]
#[allow(async_fn_in_trait)]
pub trait TlsHandshaker {
    type Connection;
    type HandshakeError: Debug;
    async fn handshake<S: AsyncTransport>(
        &self,
        server_name: &str,
        stream: &mut S,
    ) -> Result<Self::Connection, Self::HandshakeError>
    where
        S::Error: Debug;
}
