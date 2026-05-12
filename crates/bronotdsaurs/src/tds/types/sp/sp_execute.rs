//! # sp_execute (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-execute-transact-sql?view=sql-server-ver17
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;

// sp_execute handle OUTPUT
//     [ , bound_param ] [ , ...n ]
// [ ; ]
#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpExecute {
    handle: ParameterData, // required; int
    bound_parameters: Vec<ParameterData>,
}

 impl SpExecute {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let mut parameters = vec![self.handle];
        parameters.extend(self.bound_parameters);
    
        builder
            .all_headers(all_headers)
            .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpExecute))
            .parameter_data(parameters)
            .option_flags(OptionFlags::default());

        #[cfg(feature = "tds7.4")]
        builder
            .enclave_package(vec![]);

        builder
            .build()
            .unwrap()
    }
}