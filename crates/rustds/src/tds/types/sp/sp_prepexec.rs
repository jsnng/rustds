//! # sp_prepexecute (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-execute-transact-sql?view=sql-server-ver17
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpPrepExec {
    handle: ParameterData, // required
    parameters: Option<String>, // ntext, nchar, nvarchar
    stmt: String,
    bound_parameters: Option<Vec<ParameterData>>,
}

impl SpPrepExec {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut handle = self.handle;
        handle.param_meta_data.status_flags = StatusFlags::new(StatusFlags::OUTPUT_BY_REF, false, false);
        let mut builder = RPCReqBatchBuilder::default();
        let stmt = ParameterData::nvarchar("@stmt", &self.stmt);
        let params = ParameterData::nvarchar("@params", self.parameters.as_deref().unwrap_or(""));

        let mut parameters = vec![handle, params, stmt];
        if let Some(bound_parameter) = self.bound_parameters {
            parameters.extend(bound_parameter);
        }

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpPrepExec))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}
