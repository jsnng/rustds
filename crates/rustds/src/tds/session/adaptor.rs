use crate::tds::session::prelude::*;
use crate::tds::prelude::*;
use std::io::{Read, Write};

/// a simple ring-less read/write buffer.
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
/// bridges the TDS framing and TLS.
/// TLS expects raw `Read/Write` streams however, TLS handshake bytes are wrapped inside the TDS packet headers. So:
/// - `Read`: remove the TDS header and provide the TLS the payload.
/// - `Write`: wrap the TDS header around the encrypted bytes then send.
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct TransportAdaptor<'a, T> {
    pub transport: &'a mut T,
    pub reader: TransportAdaptorBuffer,
    pub writer: TransportAdaptorBuffer,
}

#[cfg(feature = "std")]
impl<T: Transport> Read for TransportAdaptor<'_, T>
where
    <T as Transport>::Error: core::fmt::Debug,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.reader.cursor >= self.reader.length {
            let mut header = [0u8; PreLoginHeader::LENGTH];
            read_n_bytes(self.transport, &mut header)?;

            let span = PreLoginSpan::new(&header).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid TDS header")
            })?;
            let payload_length = span.header().length() as usize - PreLoginHeader::LENGTH;
            read_n_bytes(self.transport, &mut self.reader.buffer[..payload_length])?;

            self.reader.length = payload_length;
            self.reader.cursor = 0;
        }

        let n = buf.len().min(self.reader.length - self.reader.cursor);
        buf[..n].copy_from_slice(&self.reader.buffer[self.reader.cursor..self.reader.cursor + n]);
        self.reader.cursor += n;
        Ok(n)
    }
}

#[cfg(feature = "std")]
impl<T: Transport> Write for TransportAdaptor<'_, T>
where
    <T as Transport>::Error: core::fmt::Debug,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = buf.len().min(self.writer.buffer.len() - self.writer.length);
        self.writer.buffer[self.writer.length..self.writer.length + n].copy_from_slice(&buf[..n]);
        self.writer.length += n;
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.writer.length == 0 {
            return Ok(());
        }
        let length = (PreLoginHeader::LENGTH + self.writer.length) as u16;
        let header: [u8; PreLoginHeader::LENGTH] = PreLoginHeaderBuilder::default()
            .length(length)
            .build()
            .unwrap_or_default()
            .as_bytes();
        write_n_bytes(self.transport, &header)?;
        write_n_bytes(self.transport, &self.writer.buffer[..self.writer.length])?;
        self.writer.length = 0;
        Ok(())
    }
}

#[cfg(feature = "std")]
/// write until `buf` slice is empty.
/// Returns [`std::io::ErrorKind::WriteZero`] if the SQL server closes the connection before the buffer is empty.
fn write_n_bytes<T: Transport>(transport: &mut T, buf: &[u8]) -> std::io::Result<()>
where
    <T as Transport>::Error: core::fmt::Debug,
{
    let mut offset = 0;
    while offset < buf.len() {
        let n = transport
            .write(&buf[offset..])
            .map_err(|e| std::io::Error::other(format!("transport write error: {:?}", e)))?;
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
/// read until `buf` slice is filled.
/// Returns [`std::io::ErrorKind::UnexpectedEof`] if the SQL server stops sending before the slice is filled, 
fn read_n_bytes<T: Transport>(transport: &mut T, buf: &mut [u8]) -> std::io::Result<()>
where
    <T as Transport>::Error: core::fmt::Debug,
{
    let mut offset = 0;
    while offset < buf.len() {
        let n = transport
            .read(&mut buf[offset..])
            .map_err(|e| std::io::Error::other(format!("transport read error: {:?}", e)))?;

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
