//! TransportAdaptor - TDS 7.x only
use crate::tds::session::prelude::*;
use crate::tds::prelude::*;

/// a simple ring-less read buffer.
/// used for framing TDS packets over tls.
#[derive(Debug, Clone)]
pub struct TransportAdaptorBuffer {
    pub buffer: [u8; MAX_TDS_PACKET_BYTES],
    pub length: usize,
    pub cursor: usize,
}

impl Default for TransportAdaptorBuffer {
    fn default() -> Self {
        Self {
            buffer: [0u8; MAX_TDS_PACKET_BYTES],
            length: 0,
            cursor: 0,
        }
    }
}

/// bridges TDS framing and TLS, async edition.
/// TLS handshake bytes are wrapped inside TDS packet headers, so:
/// - `read`: strip the TDS header and yield the payload to TLS.
/// - `write`: wrap each TLS record (whatever rustls hands us in one call) in a TDS header and send.
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct TransportAdaptor<'a, T> {
    pub transport: &'a mut T,
    pub reader: TransportAdaptorBuffer,
}

#[cfg(feature = "std")]
impl<T: AsyncTransport> AsyncTransport for TransportAdaptor<'_, T> {
    type Error = std::io::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if self.reader.cursor >= self.reader.length {
            let mut header = [0u8; PreLoginHeader::LENGTH];
            read_n_bytes(&mut *self.transport, &mut header).await?;

            let span = PreLoginSpan::new(&header).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid TDS header")
            })?;
            let payload_length = span.header().length() as usize - PreLoginHeader::LENGTH;
            read_n_bytes(&mut *self.transport, &mut self.reader.buffer[..payload_length]).await?;

            self.reader.length = payload_length;
            self.reader.cursor = 0;
        }

        let n = buf.len().min(self.reader.length - self.reader.cursor);
        buf[..n].copy_from_slice(&self.reader.buffer[self.reader.cursor..self.reader.cursor + n]);
        self.reader.cursor += n;
        Ok(n)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let length = (PreLoginHeader::LENGTH + buf.len()) as u16;
        let header: [u8; PreLoginHeader::LENGTH] = PreLoginHeaderBuilder::default()
            .length(length)
            .build()
            .unwrap_or_default()
            .as_bytes();
        write_n_bytes(&mut *self.transport, &header).await?;
        write_n_bytes(&mut *self.transport, buf).await?;
        Ok(buf.len())
    }

    fn set_read_timeout(
        &mut self,
        timeout: Option<core::time::Duration>,
    ) -> Result<(), Self::Error> {
        self.transport
            .set_read_timeout(timeout)
            .map_err(|_| std::io::Error::other("transport set_read_timeout error"))
    }

    fn set_write_timeout(
        &mut self,
        timeout: Option<core::time::Duration>,
    ) -> Result<(), Self::Error> {
        self.transport
            .set_write_timeout(timeout)
            .map_err(|_| std::io::Error::other("transport set_write_timeout error"))
    }
}

#[cfg(feature = "std")]
async fn write_n_bytes<T: AsyncTransport>(transport: &mut T, buf: &[u8]) -> std::io::Result<()> {
    let mut offset = 0;
    while offset < buf.len() {
        let n = transport
            .write(&buf[offset..])
            .await
            .map_err(|_| std::io::Error::other("transport write error"))?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "connection closed",
            ));
        }
        offset += n;
    }
    Ok(())
}

#[cfg(feature = "std")]
async fn read_n_bytes<T: AsyncTransport>(transport: &mut T, buf: &mut [u8]) -> std::io::Result<()> {
    let mut offset = 0;
    while offset < buf.len() {
        let n = transport
            .read(&mut buf[offset..])
            .await
            .map_err(|_| std::io::Error::other("transport read error"))?;

        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "connection closed",
            ));
        }
        offset += n;
    }
    Ok(())
}
