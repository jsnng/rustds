//! # sp_cursorprepare (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursorprepare-transact-sql?view=sql-server-ver17
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorPrepare {
    perpared_handle: i32,
    params: Option<String>,
    stmt: String,
    options: Option<i32>,
    scroll_opts: Option<ScrollOpt>,
    cc_opts: Option<CcOpt>,
}

impl SpCursorPrepare {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let mut prepared_handle = ParameterData::int4(String::new(), self.perpared_handle);
        prepared_handle.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        let params = ParameterData::nvarchar(String::new(), &self.params.unwrap_or_default());
        let stmt = ParameterData::nvarchar(String::new(), &self.stmt);
        let mut parameters = vec![prepared_handle, params, stmt];

        if let Some(options) = self.options {
            parameters.push(ParameterData::int4(String::new(), options));
            if let Some(scroll_opts) = self.scroll_opts {
                parameters.push(ParameterData::uint4(String::new(), scroll_opts.0));
                if let Some(cc_opts) = self.cc_opts {
                    parameters.push(ParameterData::uint4(String::new(), cc_opts.0));
                }
            }
        }


        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorPrepare))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}