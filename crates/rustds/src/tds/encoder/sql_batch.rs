use crate::tds::prelude::*;
use crate::tds::session::prelude::*;

#[cfg(kani)]
use kani;

impl MessageEncoder for SQLBatch {
    type Error = EncodeError;
    type Header = SQLBatchHeader;
    fn oneshot(
        &self,
        buf: &mut SessionBuffer,
        header: &mut Self::Header,
    ) -> Result<usize, Self::Error> {
        let mut cursor = SQLBatchHeader::LENGTH;
        cursor += self.all_headers.encode(&mut buf.writeable()[cursor..]);
        for char in self.sql_text.encode_utf16() {
            buf.writeable()[cursor..][..2].copy_from_slice(&char.to_le_bytes());
            cursor += 2;
        }
        header.length = cursor as u16;
        buf.writeable()[..SQLBatchHeader::LENGTH].copy_from_slice(&header.as_bytes());
        Ok(cursor)
    }
}
