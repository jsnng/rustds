use crate::tds::prelude::*;

/// Used to decode a row span with the provided column metadata.
#[derive(Debug, Clone, Copy)]
pub struct RowSpanIter<'a> {
    pub bytes: &'a [u8],
    col_metadata_iter: ColumnMetaDataSpanIter<'a>,
}

impl<'a> RowSpanIter<'a> {
    pub fn new(bytes: &'a [u8], col_metadata: &'a ColMetaDataSpan<'a>) -> Self {
        Self {
            bytes,
            col_metadata_iter: col_metadata.into_iter(),
        }
    }

    pub fn from_owned(bytes: &'a [u8], col_metadata: &'a ColMetaDataOwned) -> Self {
        Self {
            bytes,
            col_metadata_iter: col_metadata.into_iter(),
        }
    }

    pub fn all_column_data(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> Iterator for RowSpanIter<'a> {
    type Item = RowItemSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let col = self.col_metadata_iter.next()?;
        let bytes = to_dtype_bytes(&mut self.bytes, col.ty())?;
        Some(RowItemSpan { bytes })
    }
}
