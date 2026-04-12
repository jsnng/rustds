use crate::tds::prelude::*;

#[derive(Debug, Clone)]
pub struct BulkLoadBCP {
    _bulk_load_metadata: ColMetaDataToken,
    _bulk_load_rows: Vec<RowToken>,
}

#[derive(Debug, Clone)]
pub struct BulkLoadUTWT {
    _bulk_data: String, // L_VARBTYE
}

impl BulkLoadBCP {}

impl BulkLoadUTWT {}
