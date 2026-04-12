//! # sp_cursorclose (Transact-SQL)
//!
//! <https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursorclose-transact-sql?view=sql-server-ver17>
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorClose {
    cursor: i32,
}

impl SpCursorClose {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();
        let cursor  = ParameterData::cursor(String::new(), self.cursor);
        let parameters = vec![cursor];


        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorClose))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}