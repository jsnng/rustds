use core::time::Duration;

/// 3.2.2 Timers
/// - Connection Timer. Maximum time spent during the establishment of a TDS connection. Default should be 15 seconds.
/// - Client Request Timer. Maximum time waiting for a query response from the server after a connection has been established. Default is implementation specific.
/// - Cancel Timer. Maximum time waiting for a query cancellation acknowledgement after an Attention request is sent to a server. Default is implementation specific.

#[derive(Debug)]
pub struct Timers {
    pub connection: Option<Duration>,
    pub request: Option<Duration>,
    pub cancel: Option<Duration>,
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            connection: Some(Duration::from_secs(15)),
            request: None,
            cancel: None,
        }
    }
}
