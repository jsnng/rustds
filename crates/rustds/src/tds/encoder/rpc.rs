use crate::tds::prelude::*;

impl MessageEncoder for RPCReqBatch {
    type Error = EncodeError;

    type Header = RPCReqBatchHeader;

    fn oneshot(
        &self,
        buf: &mut crate::tds::session::prelude::SessionBuffer,
        header: &mut Self::Header,
    ) -> Result<usize, Self::Error> {
        let mut cursor = RPCReqBatchHeader::LENGTH;
        cursor += self.all_headers.encode(&mut buf.writeable()[cursor..]);

        cursor += self.name_len_proc_id.encode(&mut buf.writeable()[cursor..]);
        wint!(buf.writeable(), cursor, u16, self.option_flags.as_bytes());
        #[cfg(feature = "tds7.4")] {
            let length = self.enclave_package.len();
            if length > 0 {
                wint!(buf.writeable(), cursor, u16, length);
                wvec!(buf.writeable(), cursor, self.enclave_package);
            }
        }

        if !self.parameter_data.is_empty() {
            for parameter_data in &self.parameter_data {
                cursor += parameter_data.encode(&mut buf.writeable()[cursor..]);
            }
        }

        header.length = cursor as u16;
        buf.writeable()[..RPCReqBatchHeader::LENGTH].copy_from_slice(&header.as_bytes());
        Ok(cursor)
    }
}

impl Encoder for NameLenProcId {
    fn encode(&self, buf: &mut [u8]) -> usize {
        match self {
            NameLenProcId::ProcName(x) => {
                let mut cursor = size_of::<u16>();
                let mut chars = 0u16;
                for char in x.0.encode_utf16() {
                    buf[cursor..][..2].copy_from_slice(&char.to_le_bytes());
                    cursor += 2;
                    chars += 1;
                }
                buf[..size_of::<u16>()].copy_from_slice(&chars.to_le_bytes());
                cursor
            }
            NameLenProcId::ProcID(x) => {
                buf[..size_of::<u16>()].copy_from_slice(&0xffff_u16.to_le_bytes());
                buf[size_of::<u16>()..size_of::<u16>()*2].copy_from_slice(&(*x as u16).to_le_bytes());
                size_of::<u16>() * 2
            }
        }
    }
}

#[derive(Debug)]
struct PartiallyLengthPrefixedDataType<'a>(&'a [u8]);

#[allow(unused)]
impl PartiallyLengthPrefixedDataType<'_> {
    pub const NULL: u64 = 0xffff_ffff_ffff_ffff_u64;
    pub const UNKNOWN_LENGTH: u64 = 0xffff_ffff_ffff_fffe_u64;
}

impl<'a> Encoder for PartiallyLengthPrefixedDataType<'a> {
    fn encode(&self, buf: &mut [u8]) -> usize {
        let bytes = self.0;

        let mut cursor = 0;
        wint!(buf, cursor, u64, bytes.len());

        if !bytes.is_empty() {
            wint!(buf, cursor, u32, bytes.len());
            wvec!(buf, cursor, bytes);
        }

        wint!(buf, cursor, u32, 0u32);
        cursor
    }
}

impl Encoder for ParameterData {
    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut cursor = self.param_meta_data.encode(buf);
        match self.param_meta_data.type_info.dtype_max_len {
            None => {
                wvec!(buf, cursor, self.param_len_data);
            }
            Some(TypeInfoVarLen::Byte(_)) => {
                wint!(buf, cursor, u8, self.param_len_data.len());
                wvec!(buf, cursor, self.param_len_data);
            }
            Some(TypeInfoVarLen::Ushort(0xffff)) => {
                cursor += PartiallyLengthPrefixedDataType(&self.param_len_data).encode(&mut buf[cursor..]);
            }
            Some(TypeInfoVarLen::Ushort(_)) => {
                wint!(buf, cursor, u16, self.param_len_data.len());
                wvec!(buf, cursor, self.param_len_data);
            }
            Some(TypeInfoVarLen::Long(_)) => {
                wint!(buf, cursor, u32, self.param_len_data.len());
                wvec!(buf, cursor, self.param_len_data);
            }
        }
        for param_cipher_info in &self.param_cipher_info {
                cursor += param_cipher_info.encode(&mut buf[cursor..]);
        }
        cursor
    }
}

impl Encoder for ParamMetaData {
    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut cursor = size_of::<u8>();
        let mut chars = 0u8;
        for char in self.name.encode_utf16() {
            buf[cursor..][..2].copy_from_slice(&char.to_le_bytes());
            cursor += 2;
            chars += 1;
        }
        buf[0] = chars;
        wint!(buf, cursor, u8, self.status_flags.0);

        cursor += self.type_info.encode(&mut buf[cursor..]);
        cursor
    }
}

impl Encoder for ParamCipherInfo {
    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut cursor = self.ty.encode(&mut buf[0..]);
        wint!(buf, cursor, u8, self.encryption_algo);
        if let Some(algo_name) = &self.algo_name {
            let offset = cursor;
            cursor += size_of::<u8>();
            let mut chars = 0u8;
            for char in algo_name.encode_utf16() {
                buf[cursor..][..2].copy_from_slice(&char.to_le_bytes());
                cursor += 2;
                chars += 1;
            }
            buf[offset] = chars;
        } else {
            wint!(buf, cursor, u8, 0);
        }

        wint!(buf, cursor, u8, self.encryption_type);
        wint!(buf, cursor, u32, self.database_id);
        wint!(buf, cursor, u32, self.cek_id);
        wint!(buf, cursor, u32, self.cek_version);
        wint!(buf, cursor, u64, self.cek_md_version);
        wint!(buf, cursor, u8, self.norm_version);
        cursor
    }
}