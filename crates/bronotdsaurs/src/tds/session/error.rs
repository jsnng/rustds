use crate::tds::prelude::*;

pub enum TransportError<E> {
    InnerError(E),
    UnexpectedRead(),
    UnexpectedWrite(String),
}

impl<E: core::fmt::Display> core::fmt::Display for TransportError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TransportError::InnerError(e) => write!(f, "Inner error: {}", e),
            TransportError::UnexpectedRead() => write!(f, "Unexpected read"),
            TransportError::UnexpectedWrite(msg) => write!(f, "Unexpected write: {}", msg),
        }
    }
}

impl<E: core::fmt::Debug + core::fmt::Display> core::fmt::Debug for TransportError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

pub struct HexDump {
      pub bytes: [u8; 32],
      pub length: usize,
  }

pub enum SessionError {
    DecodeError(DecodeError),
    EncodeError(EncodeError),
    InvalidPacketType { got: u8 },
    UnexpectedEndOfStream {
        head: usize,
        tail: usize,
        eof: bool,
        peek: Option<u8>,
        hexdump: HexDump,
    },
    InvalidPacketLength {
        got: usize,
        expected: usize,
    },
    LoginFailed,
    MappedError(String),
    PartialRead,
    ServerClosedTransportConnection,
    TransportReadError(&'static core::panic::Location<'static>),
    TransportWriteError(&'static core::panic::Location<'static>),
    TransportTimeoutError,
    Unimplemented,
    RequestedPacketSizeTooLarge,
    BufferIndexOutOfBoundsError(String),
}

impl SessionError {
    #[track_caller]
    pub fn transport_read_error() -> Self {
        Self::TransportReadError(core::panic::Location::caller())
    }
    #[track_caller]
    pub fn transport_write_error() -> Self {
        Self::TransportWriteError(core::panic::Location::caller())
    }
}

impl core::fmt::Debug for SessionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for SessionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DecodeError(x) => write!(f, "Deserialisation error: {}", x),
            Self::EncodeError(x) => write!(f, "Serialisation error: {}", x),
            Self::InvalidPacketType { got } => write!(f, "Invalid packet type: 0x{:02x}", got),
            Self::InvalidPacketLength { expected, got } => write!(f, "Invalid packet length: {expected}, {got}"),
            Self::UnexpectedEndOfStream { head, tail, eof, peek, hexdump } =>
                write!(f, "UnexpectedEndOfStream: (head={head} tail={tail} eof={eof} peek={peek:02x?}) bytes={hexdump}"),
            Self::LoginFailed => write!(f, "Login failed"),
            Self::MappedError(msg) => write!(f, "{}", msg),
            Self::PartialRead => write!(f, "Partial read"),
            Self::ServerClosedTransportConnection => write!(f, "Server closed connection"),
            Self::TransportReadError(loc) => write!(f, "Transport read error at {}:{}", loc.file(), loc.line()),
            Self::TransportWriteError(loc) => write!(f, "Transport write error at {}:{}", loc.file(), loc.line()),
            Self::TransportTimeoutError => write!(f, "Transport timeout error"),
            Self::Unimplemented => write!(f, "Not supported (yet)"),
            Self::RequestedPacketSizeTooLarge => {
                write!(f, "Server requested packet size not supported.")
            }
            Self::BufferIndexOutOfBoundsError(x) => write!(f, "BufferIndexOutOfBounds: {}", x),
        }
    }
}

impl core::fmt::Display for HexDump {                                                                            
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {                                     
        for b in &self.bytes[..self.length] {                                                                    
            write!(f, "{:02x} ", b)?;
        }                                                                                                        
        Ok(())                                                                                                 
    }                                                                                                            
}                                                                                                              
       
impl core::error::Error for SessionError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::DecodeError(e) => Some(e),
            Self::EncodeError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<crate::tds::encoder::error::EncodeError> for SessionError {
    #[track_caller]
    fn from(e: crate::tds::encoder::error::EncodeError) -> Self {
        SessionError::EncodeError(e)
    }
}

impl From<crate::tds::decoder::error::DecodeError> for SessionError {
    #[track_caller]
    fn from(e: crate::tds::decoder::error::DecodeError) -> Self {
        SessionError::DecodeError(e)
    }
}
