//! # sp_cursorfetch (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursorfetch-transact-sql?view=sql-server-ver17
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorFetch {
    cursor: i32,
    fetch_type: FetchType,
    row_num: Option<i32>,
    n_rows: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct FetchType(pub u32);
impl FetchType {
    pub const FIRST: Self = Self(0x0001);
    pub const NEXT: Self = Self(0x0002);
    pub const PREV: Self = Self(0x0004);
    pub const LAST: Self = Self(0x0008);
    pub const ABSOLUTE: Self = Self(0x10);
    pub const RELATIVE: Self = Self(0x20);
    pub const REFRESH: Self = Self(0x80);
    pub const INFO: Self = Self(0x100);
    pub const PREV_NOADJUST: Self = Self(0x200);
    pub const SKIP_UPDT_CNC: Self = Self(0x400);
}

impl core::ops::BitOr for FetchType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl SpCursorFetch {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let cursor = ParameterData::cursor(String::new(), self.cursor);
        let fetch_type = ParameterData::uint4(String::new(), self.fetch_type.0);
        let mut parameters = vec![cursor, fetch_type];
        if let Some(row_num) = self.row_num {
            parameters.push(ParameterData::int4(String::new(), row_num));
            parameters.push(ParameterData::int4(String::new(), self.n_rows.unwrap_or(20)));
        }

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorFetch))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}