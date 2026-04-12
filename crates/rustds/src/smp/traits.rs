#![allow(dead_code)]
use crate::smp::types::ControlFlagType;

pub(in crate::smp) trait SMPHeaderFields {
    fn smid(&self) -> u8;
    fn flag(&self) -> Option<ControlFlagType>;
    fn sid(&self) -> u16;
    fn length(&self) -> u32;
    fn seq_num(&self) -> u32;
    fn wndw(&self) -> u32;
}

pub trait Encode {
    type Error;
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait Decode<'a> {
    type Error;
    fn decode(buf: &'a [u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
