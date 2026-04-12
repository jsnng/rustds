use crate::tds::prelude::*;


impl<'a> SessionStatusSpan<'a> {}

#[derive(Debug, Clone, Copy)]
pub struct SessionStatusToken {
    pub(crate) _ty: u8,
    pub(crate) _length: u32,
    pub(crate) _status: u8,
    pub(crate) _session_state_dataset: SessionStatusDataset,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionStatusDataset;
