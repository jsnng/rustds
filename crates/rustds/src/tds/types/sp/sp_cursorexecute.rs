//! # sp_cursorexecute (Transact-SQL)
//!
//! <https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursorexecute-transact-sql?view=sql-server-ver17>
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorExecute {
    perpared_handle: i32,
    cursor: i32,
    scroll_opt: Option<ScrollOpt>,
    cc_opt: Option<CcOpt>,
    row_count: Option<i32>,
}

impl SpCursorExecute {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let perpared_handle = ParameterData::int4(String::new(), self.perpared_handle);
        let cursor = ParameterData::cursor(String::new(), self.cursor);
        let mut parameters = vec![perpared_handle, cursor ];

        if let Some(scrollopt) = self.scroll_opt {
            parameters.push(ParameterData::uint4(String::new(), scrollopt.0));
            if let Some(ccopt) = self.cc_opt {
                parameters.push(ParameterData::uint4(String::new(), ccopt.0));
            }
        }
        let mut row_count = ParameterData::int4(String::new(), self.row_count.unwrap_or(20));
        row_count.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        
        parameters.push(row_count);
        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorExecute))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}