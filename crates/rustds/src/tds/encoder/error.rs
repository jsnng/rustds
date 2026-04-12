use core::panic::Location;

#[derive(Debug)]
pub enum EncodeError {
    BufferTooSmall {
        required: usize,
        available: usize,
        location: &'static Location<'static>,
    },
    InvalidField {
        location: &'static Location<'static>,
    },
    PreviousRowNotFlushed,
}

impl EncodeError {
    #[track_caller]
    pub fn buffer_too_small(required: usize, available: usize) -> Self {
        Self::BufferTooSmall {
            required,
            available,
            location: Location::caller(),
        }
    }

    pub fn invalid_field() -> Self {
        Self::InvalidField {
            location: Location::caller(),
        }
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall {
                required,
                available,
                location,
            } => {
                write!(
                    f,
                    "Buffer too small: required {} bytes, available {} bytes @ {}",
                    required, available, location
                )
            }
            Self::InvalidField { location } => write!(f, "Invalid field @ {}", location),
            Self::PreviousRowNotFlushed => write!(f, "Previous row not flushed"),
        }
    }
}

impl core::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}
