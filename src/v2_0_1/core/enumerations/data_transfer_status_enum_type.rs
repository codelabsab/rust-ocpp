#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum DataTransferStatusEnumType {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
