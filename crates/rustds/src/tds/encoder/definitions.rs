use crate::tds::prelude::*;

impl Encoder for TypeInfo {
    fn encode(&self, buf: &mut [u8]) -> usize {
        buf[0] = u8::from(self.dtype);
        let mut cursor = 1;
        if let Some(dtype_max_length) = self.dtype_max_len {
            match dtype_max_length {
                TypeInfoVarLen::Byte(x) => {
                    wint!(buf, cursor, u8, x);
                }
                TypeInfoVarLen::Ushort(x) => {
                    wint!(buf, cursor, u16, x);
                }
                TypeInfoVarLen::Long(x) => {
                    wint!(buf, cursor, u32, x);
                }
            }
        }
        if let Some(collation) = self.collation {
            cursor += collation.encode(&mut buf[cursor..cursor + size_of::<u32>() + size_of::<u8>()])
        }

        if let Some(precision) = self.precision {
            wint!(buf, cursor, u8, precision);
        }

        if let Some(scale) = self.scale {
            wint!(buf, cursor, u8, scale);
        }

        cursor
    }
}

impl Encoder for Collation {
    fn encode(&self, buf: &mut [u8]) -> usize {
        let bytes: u32 = self.lcid & 0xfffff
            | (self.f_ignore_case as u32) << 20
            | (self.f_ignore_accent as u32) << 21
            | (self.f_ignore_width as u32) << 22
            | (self.f_ignore_kana as u32) << 23
            | (self.f_binary as u32) << 24
            | (self.f_binary2 as u32) << 25
            | (self.f_utf8 as u32) << 26
            | (self.version as u32) << 28;
        let mut cursor = 0;
        wint!(buf, cursor, u32, bytes);
        wint!(buf, cursor, u8, self.sort_id);
        cursor
    }
}
