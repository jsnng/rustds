//! # sp_cursoroption (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursoroption-transact-sql?view=sql-server-ver17
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorOption {
    cursor: i32,
    value: CodeValue,
}


#[repr(u32)]
#[derive(Debug, Clone)]
pub enum CodeValue {
    TextPtrOnly(u32) = 0x0001,
    CursorName(String) = 0x0002,
    TextData(u32) = 0x0003,
    ScrollOpt(ScrollOpt) = 0x0004,
    CcOpt(CcOpt) = 0x0005,
    RowCount(i32) = 0x0006,
}

impl CodeValue {
    pub fn code(&self) -> u32 {
        unsafe { *<*const _>::from(self).cast::<u32>() }
    }
}

impl SpCursorOption {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();
        let cursor = ParameterData::cursor(String::new(), self.cursor);
        let code = ParameterData::uint4(String::new(), self.value.code());
        let value = match self.value {
            CodeValue::TextPtrOnly(v) => ParameterData::uint4(String::new(), v),
            CodeValue::CursorName(s) => ParameterData::nvarchar(String::new(), &s),
            CodeValue::TextData(v) => ParameterData::uint4(String::new(), v),
            CodeValue::ScrollOpt(s) => ParameterData::uint4(String::new(), s.0),
            CodeValue::CcOpt(c) => ParameterData::uint4(String::new(), c.0),
            CodeValue::RowCount(r) => ParameterData::int4(String::new(), r),
        };
        let parameters = vec![cursor, code, value];


        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorOption))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}