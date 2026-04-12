#![allow(dead_code)]

#[cfg(kani)]
extern crate kani;

use crate::smp::traits::SMPHeaderFields;

pub struct SMPHeaderSpan<'a> {
    bytes: &'a [u8],
}

#[repr(transparent)]
pub struct SMPHeaderU128(u128);

pub struct SMPHeader {
    pub smid: u8,
    pub flag: ControlFlagType,
    pub sid: u16,
    pub length: u32,
    pub seq_num: u32,
    pub wndw: u32,
}

impl SMPHeader {
    pub const LENGTH: usize = 16;
    pub const SEQ_NUM_START: u32 = 0x00;
}

#[repr(u8)]
pub enum ControlFlagType {
    Syn = 0x01,
    Ack = 0x02,
    Fin = 0x04,
    Data = 0x08,
}

/// all integer fields are represented in little-endian byte order.
#[rustfmt::skip]
impl<'a> SMPHeaderFields for SMPHeaderSpan<'a> {
    #[inline]
    fn smid(&self) -> u8 { self.bytes[0] }
    #[inline]
    fn flag(&self) -> Option<ControlFlagType> {
        match self.bytes[1] {
            0x01 => Some(ControlFlagType::Syn),
            0x02 => Some(ControlFlagType::Ack),
            0x04 => Some(ControlFlagType::Fin),
            0x08 => Some(ControlFlagType::Data),
            _ => None,
        }
    }
    #[inline]
    fn sid(&self) -> u16 { u16::from_le_bytes([self.bytes[2], self.bytes[3]]) }
    #[inline]
    fn length(&self) -> u32 { u32::from_le_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]])}
    #[inline]
    fn seq_num(&self) -> u32 { u32::from_le_bytes([self.bytes[8], self.bytes[9], self.bytes[10], self.bytes[11]])}
    #[inline]
    fn wndw(&self) -> u32 { u32::from_le_bytes([self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]])}
}

#[rustfmt::skip]
impl SMPHeaderFields for SMPHeaderU128 {
    #[inline]
    fn smid(&self) -> u8 { (self.0 & 0xff) as u8 }
    #[inline]
    fn flag(&self) -> Option<ControlFlagType> {
        match ((self.0 >> 8) & 0xFF) as u8 {
            0x01 => Some(ControlFlagType::Syn),
            0x02 => Some(ControlFlagType::Ack),
            0x04 => Some(ControlFlagType::Fin),
            0x08 => Some(ControlFlagType::Data),
            _ => None,
        }
    }
    #[inline]
    fn sid(&self) -> u16 { (self.0 >> 16 & 0xffff) as u16 }
    #[inline]
    fn length(&self) -> u32 { (self.0 >> 32 & 0xffffffff) as u32 }
    #[inline]
    fn seq_num(&self) -> u32 { (self.0 >> 64 & 0xffffffff) as u32 }
    #[inline]
    fn wndw(&self) -> u32 { (self.0 >> 96 & 0xffffffff) as u32 }
}
