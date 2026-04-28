//! # sp_prepare (Transact SQL)
//!
//! <https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-prepare-transact-sql?view=sql-server-ver17>
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SpPrepare {
    handle: ParameterData,
    stmt: String, // ntext, nchar, nvarchar
    parameters: Option<String>,
    option_flags: Option<OptionFlags>, // optional, options requires 0x0001 RETURN_METADATA
}

 impl SpPrepare {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let stmt = ParameterData::nvarchar(String::new(), &self.stmt);
        let params =  ParameterData::nvarchar("", "");

        let mut parameters = vec![self.handle, params];
        if let Some(param) = self.parameters {
            let parameter = ParameterData::nvarchar(String::new(), param.as_str());
            parameters.push(parameter);
        }
        parameters.push(stmt);
        if let Some(option_flags) = self.option_flags {
            let flag = ParameterData::option_flags(String::new(),option_flags);
            parameters.push(flag);
        }

        builder
            .all_headers(all_headers)
            .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpPrepare))
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
