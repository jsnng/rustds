//! # sp_unprepare (Transact-SQL)
//!
//! https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-unprepare-transact-sql?view=sql-server-ver17
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpUnprepare {
    handle: ParameterData, // required; int
}

impl SpUnprepare {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();
        let parameters = vec![self.handle];

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
        .all_headers(all_headers)
        .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpUnprepare))
        .option_flags(OptionFlags::default())
        .parameter_data(parameters)
        .build()
        .unwrap()
    }
}