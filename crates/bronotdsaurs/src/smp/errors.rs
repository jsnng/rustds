#[derive(Debug)]
pub enum EncodeError {
    BufferTooSmall { required: usize, available: usize },
    InvalidField,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidPacketType,
    InvalidLength,
    UnexpectedEnd,
    InvalidField,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall {
                required,
                available,
            } => {
                write!(
                    f,
                    "Buffer too small: required {} bytes, available {} bytes",
                    required, available
                )
            }
            Self::InvalidField => write!(f, "Invalid field"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPacketType => write!(f, "Invalid packet type"),
            Self::InvalidLength => write!(f, "Invalid length"),
            Self::UnexpectedEnd => write!(f, "Unexpected end of data"),
            Self::InvalidField => write!(f, "Invalid field"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}
