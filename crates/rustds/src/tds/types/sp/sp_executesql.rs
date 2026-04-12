//! sp_executesql (Transact-SQL)
//!
//! <https://learn.microsoft.com/en-us/sql/relational-databases/system-stored-procedures/sp-executesql-transact-sql?view=sql-server-ver18>
use derive_builder::Builder;
use crate::tds::types::sp::prelude::*;
use alloc::format;

#[derive(Debug, Clone, Default, Builder)]
#[builder(no_std, setter(strip_option), default)]
pub struct SpExecuteSql {
    stmt: String,
    parameters: Option<Vec<ParameterData>>,
    option_flags: Option<OptionFlags>,
}

impl SpExecuteSql {
    pub fn into_rpc_batch(self, all_headers: AllHeaders) -> RPCReqBatch {
        let mut builder = RPCReqBatchBuilder::default();

        let stmt = ParameterData::nvarchar(String::new(), &self.stmt);

        builder
            .all_headers(all_headers)
            .name_len_proc_id(NameLenProcId::ProcID(ProcId::SpExecuteSql))
            .option_flags(
                self.option_flags
                    .unwrap_or_else(|| OptionFlags::new(false, false, false)),
            );

        let mut parameters = vec![stmt];
        if let Some(additional) = self.parameters {
            let declaration = additional
            .iter()
            .map(|p| format!("{} {}", p.param_meta_data.name, p.param_meta_data.type_info.to_tsql()))
            .collect::<Vec<_>>()
            .join(", ");

            parameters.push(ParameterData::nvarchar(String::new(), &declaration));
            parameters.extend(additional);
        } else {
            parameters.push(ParameterData::nvarchar("", ""));
        }

        #[cfg(feature = "tds7.4")]
        builder.enclave_package(vec![]);

        builder
            .parameter_data(parameters)
            .build()
            .unwrap()
    }
}
