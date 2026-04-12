use crate::tds::prelude::*;

#[derive(Debug, Clone)]
pub struct SspiToken {
    pub(crate) _ty: u8,
    pub(crate) _sspi_buffer: String,
}

impl<'a> SspiSpan<'a> {}
