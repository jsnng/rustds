use crate::tds::prelude::*;

tds_packet_header!(SQLBatchHeader, ClientMessageType::SQLBatch);

#[derive(Debug, Clone, Builder)]
#[builder(no_std, setter(strip_option))]
pub struct SQLBatch {
    pub(crate) all_headers: AllHeaders,
    #[cfg(feature = "tds8.0")]
    pub(crate) enclave_package: u8,
    pub(crate) sql_text: String,
}

#[cfg(feature = "tds8.0")]
pub struct EnclavePackage;
