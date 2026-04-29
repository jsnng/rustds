use transport::AsyncTransport;
use crate::tds::types::traits::TDSPacketHeader;

/// `Streamer` encodes a message struct `M` and uses `write` to transport it.
pub trait Streamer<M, H: TDSPacketHeader, T: AsyncTransport> {
    type Error;
    fn stream(&mut self, msg: M) -> Result<(), Self::Error>;
    fn header(&mut self, header: H) -> Result<(), Self::Error>;
}

pub trait Observer<Signal> {
    fn on(&mut self, event: &Signal);
}

// No-op observer for backward compatibility
impl<S> Observer<S> for () {
    fn on(&mut self, _event: &S) {}
}
