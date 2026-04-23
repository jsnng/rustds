use alloc::string::String;
use alloc::format;

#[derive(Debug)]
pub enum DecodeError {
    KaniStubError,
    InvalidField(String),
    InvalidLength(String),
    InvalidPacketType(String),
    UnexpectedEof(String),
    InvalidData(String),
    InvalidDataTokenType(String),
    InvalidEnvChangeType(String),
}

impl DecodeError {
    #[track_caller]
    pub fn invalid_field(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidField(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn invalid_length(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidLength(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn invalid_packet_type(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidPacketType(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn unexpected_eof(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::UnexpectedEof(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn invalid_data(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidData(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn invalid_data_token_type(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidDataTokenType(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
    #[track_caller]
    pub fn invalid_env_change_type(err_msg: String) -> Self {
        let loc = core::panic::Location::caller();
        Self::InvalidEnvChangeType(format!("[{}:{}] {}", loc.file(), loc.line(), err_msg))
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::KaniStubError => write!(f, ""),
            DecodeError::InvalidField(x) => write!(f, "Invalid field: {}", x),
            DecodeError::InvalidLength(x) => write!(f, "Invalid length: {}", x),
            DecodeError::InvalidPacketType(x) => write!(f, "Invalid packet type: {}", x),
            DecodeError::UnexpectedEof(x) => write!(f, "Unexpected end of data: {}", x),
            DecodeError::InvalidData(x) => write!(f, "Invalid data: {}", x),
            DecodeError::InvalidDataTokenType(x) => write!(f, "Invalid data token type: {}", x),
            DecodeError::InvalidEnvChangeType(x) => write!(f, "Invalid env change type: {}", x),
        }
    }
}

impl core::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}
