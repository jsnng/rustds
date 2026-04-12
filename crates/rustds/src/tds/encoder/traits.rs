use crate::tds::session::SessionBuffer;

pub trait MessageEncoder {
    type Error;
    type Header;

    // Encoder calls `buffer.writeable()` to get `&mut [u8]` and writes into it.
    // Returns the number of bytes written. Use this to advance the tail.
    fn oneshot(
        &self,
        buf: &mut SessionBuffer,
        header: &mut Self::Header,
    ) -> Result<usize, Self::Error>;
}
