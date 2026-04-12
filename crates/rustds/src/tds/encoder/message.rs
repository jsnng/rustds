use crate::tds::encoder::traits::MessageEncoder;
use crate::tds::prelude::*;
use crate::tds::session::prelude::*;
use crate::tds::types::prelude::*;

impl MessageEncoder for Attention {
    type Error = EncodeError;
    type Header = AttentionHeader;

    fn oneshot(
        &self,
        buf: &mut SessionBuffer,
        _header: &mut Self::Header,
    ) -> Result<usize, Self::Error> {
        let header = AttentionHeader::default();
        buf.writeable()[..8].copy_from_slice(&header.as_bytes());
        Ok(AttentionHeader::LENGTH)
    }
}
