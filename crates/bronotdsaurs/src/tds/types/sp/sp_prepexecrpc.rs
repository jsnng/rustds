//! # sp_prepexecrpc (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-execute-transact-sql?view=sql-server-ver17
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpPrepExecRpc {
    handle: ParameterData, // required
    rpc_call: String, // ntext
    bound_parameters: Option<Vec<ParameterData>>,
}

impl SpPrepExecRpc {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();
        let rpc_call = ParameterData::nvarchar(String::new(), &self.rpc_call);


        let mut parameters = vec![self.handle, rpc_call];
        if let Some(bound_parameter) = self.bound_parameters {
            parameters.extend(bound_parameter);
        }

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpPrepExecRpc))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}