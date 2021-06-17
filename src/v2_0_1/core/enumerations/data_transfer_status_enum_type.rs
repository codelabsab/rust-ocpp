#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DataTransferStatusEnumType {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
