#![allow(unused)]
use crate::smp::errors::{DecodeError, EncodeError};
use crate::smp::prelude::*;
use crate::smp::traits::{Decode, Encode};
use crate::smp::types::{ControlFlagType, SMPHeader};

pub struct SMPStream<'a> {
    seq_num: Option<u32>,
    current: SMPHeader,
    payload: &'a [u8],
}

impl<'a> Encode for SMPStream<'a> {
    type Error = EncodeError;
    fn encode(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut len = SMPHeader::LENGTH;
        let flag: u8 = match self.current.flag {
            ControlFlagType::Syn => 0x01,
            ControlFlagType::Ack => 0x02,
            ControlFlagType::Fin => 0x04,
            ControlFlagType::Data => {
                // since we are already here...
                // and SMP's headers are fixed-size...
                // and only Data packets have a payload...
                len = SMPHeader::LENGTH + self.payload.len();
                if buf.len() < len {
                    return Err(EncodeError::BufferTooSmall {
                        required: len,
                        available: buf.len(),
                    });
                }
                buf[SMPHeader::LENGTH..len].copy_from_slice(self.payload);
                0x08
            }
            _ => unreachable!(),
        };

        let mut seq_num = SMPHeader::SEQ_NUM_START;
        if let Some(_) = self.seq_num {
            seq_num += 1;
        }

        let len = len as u32;

        buf[0..SMPHeader::LENGTH].copy_from_slice(&[
            self.current.smid,
            flag,
            (self.current.sid & 0xff) as u8,
            (self.current.sid >> 8) as u8,
            (len & 0xff) as u8,
            (len >> 8) as u8,
            (len >> 16) as u8,
            (len >> 24) as u8,
            (seq_num & 0xff) as u8,
            (seq_num >> 8) as u8,
            (seq_num >> 16) as u8,
            (seq_num >> 24) as u8,
            (self.current.wndw & 0xff) as u8,
            (self.current.wndw >> 8) as u8,
            (self.current.wndw >> 16) as u8,
            (self.current.wndw >> 24) as u8,
        ]);

        self.seq_num = Some(seq_num);

        Ok(len as usize)
    }
}

// impl<'a> Decode<'a> for SMPStream<'a> {
//     type Error = DeserialisationError;
//     fn deserialise(buf: &'a [u8]) -> Result<Self, Self::Error>
//         where
//             Self: Sized
//     {

//     }
// }
