use crate::tds::prelude::*;

/// All server responses use packet header type 0x04 (Tabular Result).
/// See: [MS-TDS] 2.2.3.1.1
pub const SERVER_PACKET_TYPE: u8 = 0x04;

#[repr(u8)]
#[derive(Debug, Clone, Copy, TryFromIntoFormat)]
pub enum ClientMessageType {
    SQLBatch = 0x01,              // 2.2.1.4 SQL Batch
    PreTDS7Login = 0x02,          // 2.2.1.1 Pre-Login
    RemoteProcedureCall = 0x03,   // 2.2.1.6 Remote Procedure Call
    Attention = 0x06,             // 2.2.1.7 Attention
    BulkLoad = 0x07,              // 2.2.1.5 Bulk Load
    FederatedAuthenticationToken = 0x08, // 2.2.1.3 Federated Authentication Token
    TransactionManagerRequest = 0x0e,    // 2.2.1.8 Transaction Manager Request
    TDS7Login = 0x10,             // 2.2.1.2 Login
    SSPI = 0x11,
    PreLogin = 0x12,
}

tds_packet_header!(AttentionHeader, ClientMessageType::Attention);
#[derive(Debug, Clone, Copy)]
pub struct Attention;
