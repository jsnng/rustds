//! # sp_cursorprepexec (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-cursorprepexec-transact-sql?view=sql-server-ver17
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpCursorPrepExec {
    prepared_handle: i32,
    cursor: i32,
    params: Option<String>,
    stmt: String,
    scrollopt: Option<ScrollOpt>,
    ccopt: Option<CcOpt>,
    row_count: Option<i32>,
    bound_params: Vec<ParameterData>,
}

impl SpCursorPrepExec {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let mut prepared_handle = ParameterData::int4(String::new(), self.prepared_handle);
        prepared_handle.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        let mut cursor = ParameterData::cursor(String::new(), self.cursor);
        cursor.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        let params = ParameterData::nvarchar(String::new(), &self.params.unwrap_or_default());
        let stmt = ParameterData::nvarchar(String::new(), &self.stmt);
        let mut parameters = vec![prepared_handle, cursor, params, stmt];

        if let Some(scrollopt) = self.scrollopt {
            parameters.push(ParameterData::uint4(String::new(), scrollopt.0));
            if let Some(ccopt) = self.ccopt {
                parameters.push(ParameterData::uint4(String::new(), ccopt.0));
            }
        }

        let mut row_count = ParameterData::int4(String::new(), self.row_count.unwrap_or(20));
        row_count.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        parameters.push(row_count);
        parameters.extend(self.bound_params);

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpCursorPrepExec))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}
