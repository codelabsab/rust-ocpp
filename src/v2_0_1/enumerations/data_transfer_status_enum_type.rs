#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum DataTransferStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}
